//! 面向 SQLite 之外引擎的真实网络协议连接。
//!
//! SQLite 由 `rusqlite` 原生处理；对于 PostgreSQL，本模块通过纯 Rust 的
//! `postgres` 驱动讲真实网络协议，使 PostgreSQL 连接配置能真正连上服务器并
//! 执行真实 SQL。没有内置驱动的引擎仍使用离线演示数据，但连接配置会完整记录，
//! 便于日后补充驱动。

use crate::db::QueryResult; // 引入统一查询结果类型
use crate::model::{Connection, DbKind}; // 引入连接配置与引擎种类
use std::time::Instant; // 引入计时器

/// 把连接配置拼成 PostgreSQL 连接串。
fn pg_config(conn: &Connection) -> String { // 生成 libpq 风格连接串
    let mut s = format!( // 拼接基础参数
        "host={} port={} user={} dbname={} connect_timeout=6", // 键值对格式
        if conn.host.trim().is_empty() { "localhost" } else { conn.host.trim() }, // 主机（空则用 localhost）
        if conn.port == 0 { 5432 } else { conn.port }, // 端口（0 则用 5432）
        conn.username, // 用户名
        conn.database // 数据库名
    ); // format 结束
    if !conn.password.is_empty() { // 仅当存在密码时才附加
        s.push_str(&format!(" password={}", conn.password)); // 追加密码参数
    } // if 结束
    s // 返回连接串
} // pg_config 结束

/// 尝试真实连接并返回服务器版本字符串。
pub fn pg_test(conn: &Connection) -> Result<String, String> { // 测试 PostgreSQL 连接
    use postgres::{Client, NoTls}; // 使用 postgres 驱动（明文连接）
    let mut client = Client::connect(&pg_config(conn), NoTls).map_err(|e| e.to_string())?; // 建立连接，失败转字符串
    let row = client.query_one("SELECT version()", &[]).map_err(|e| e.to_string())?; // 查询服务器版本
    Ok(row.get::<_, String>(0)) // 返回版本字符串
} // pg_test 结束

/// 针对在线 PostgreSQL 服务器执行一条或多条语句，并把结果映射为与 SQLite
/// 引擎一致的 `QueryResult` 形状。
pub fn pg_run(conn: &Connection, sql: &str) -> Vec<QueryResult> { // 在 PostgreSQL 上执行 SQL
    use postgres::{Client, NoTls, SimpleQueryMessage}; // 使用 simple_query 接口
    let mut client = match Client::connect(&pg_config(conn), NoTls) { // 建立连接
        Ok(c) => c, // 连接成功
        Err(e) => { // 连接失败
            return vec![QueryResult { error: Some(format!("连接失败: {e}")), statement: sql.to_string(), ..Default::default() }]; // 返回错误结果
        } // 失败分支结束
    }; // match 结束
    let t0 = Instant::now(); // 记录开始时间
    match client.simple_query(sql) { // 执行查询
        Ok(msgs) => { // 执行成功
            let mut results: Vec<QueryResult> = Vec::new(); // 结果集合
            let mut cols: Vec<String> = Vec::new(); // 列名缓存
            let mut rows: Vec<Vec<Option<String>>> = Vec::new(); // 行数据缓存
            for m in msgs { // 遍历每条返回消息
                match m { // 分类处理消息
                    SimpleQueryMessage::Row(r) => { // 一行数据
                        if cols.is_empty() { // 首次遇到行时收集列名
                            cols = r.columns().iter().map(|c| c.name().to_string()).collect(); // 收集列名
                        } // if 结束
                        let mut rec = Vec::with_capacity(cols.len()); // 预分配一条记录
                        for i in 0..cols.len() { // 遍历每一列
                            rec.push(r.get(i).map(|s| s.to_string())); // 取值并转字符串
                        } // for 结束
                        rows.push(rec); // 压入行数据
                    } // Row 分支结束
                    SimpleQueryMessage::CommandComplete(n) => { // 一条语句执行完成
                        if !rows.is_empty() { // 若之前有查询行，先产出 SELECT 结果
                            results.push(QueryResult { // 构造 SELECT 结果
                                columns: std::mem::take(&mut cols), // 取走列名
                                rows: std::mem::take(&mut rows), // 取走行数据
                                is_select: true, // 标记为 SELECT
                                statement: sql.to_string(), // 原始语句
                                elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0, // 耗时（毫秒）
                                ..Default::default() // 其余字段默认
                            }); // push 结束
                        } // if 结束
                        results.push(QueryResult { // 产出受影响行数结果
                            affected: n as i64, // 影响行数
                            statement: sql.to_string(), // 原始语句
                            elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0, // 耗时
                            ..Default::default() // 其余默认
                        }); // push 结束
                    } // CommandComplete 分支结束
                    _ => {} // 其它消息忽略
                } // match 结束
            } // for 结束
            if !rows.is_empty() { // 处理末尾剩余的行
                results.push(QueryResult { // 构造 SELECT 结果
                    columns: cols, // 列名
                    rows, // 行数据
                    is_select: true, // 标记 SELECT
                    statement: sql.to_string(), // 语句
                    elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0, // 耗时
                    ..Default::default() // 其余默认
                }); // push 结束
            } // if 结束
            if results.is_empty() { // 若完全无结果
                results.push(QueryResult { statement: sql.to_string(), elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0, ..Default::default() }); // 至少产出一条空结果
            } // if 结束
            results // 返回结果集合
        } // Ok 分支结束
        Err(e) => vec![QueryResult { error: Some(e.to_string()), statement: sql.to_string(), ..Default::default() }], // 出错返回错误结果
    } // match 结束
} // pg_run 结束

/// 判断该引擎是否已实现真实网络驱动。
pub fn has_real_driver(kind: DbKind) -> bool { // 判断是否存在真实驱动
    matches!(kind, DbKind::PostgreSql) // 目前仅 PostgreSQL
} // has_real_driver 结束
