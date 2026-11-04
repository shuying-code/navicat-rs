//! navicat-rs 桌面程序入口。
//!
//! 职责：声明模块、提供无界面的自检（`--selftest`）、创建窗口并启动 egui 应用。

#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")] // Release 版 Windows 下不弹出黑色控制台窗口

mod app; // 应用状态与主界面
mod model; // 数据模型
mod store; // 配置持久化
mod db; // SQLite 引擎与 SQL 工具
mod realdb; // 其它引擎的真实网络驱动
mod theme; // 主题与配色
mod ui; // 各类 UI 面板

/// SQLite 引擎层的无界面功能自检。
/// 运行方式：navicat-rs.exe --selftest
fn selftest() -> i32 { // 自检入口，返回进程退出码
    use db::{build_create_table, split_sql, SqliteEngine}; // 局部引入被检测的引擎 API
    use model::{Column, ObjKind}; // 局部引入模型类型

    let mut pass = 0u32; // 通过计数
    let mut fail = 0u32; // 失败计数
    let mut check = |name: &str, ok: bool, detail: String| { // 断言闭包：打印结果并累加计数
        if ok { // 若断言通过
            pass += 1; // 通过计数 +1
            println!("[PASS] {name}  {detail}"); // 打印 PASS 行
        } else { // 否则
            fail += 1; // 失败计数 +1
            println!("[FAIL] {name}  {detail}"); // 打印 FAIL 行
        } // if 结束
    }; // 闭包结束

    let eng = match SqliteEngine::open_memory_seeded() { // 打开带演示数据的内存 SQLite 引擎
        Ok(e) => e, // 成功得到引擎
        Err(e) => { // 失败
            println!("[FATAL] cannot open seeded engine: {e}"); // 打印致命错误
            return 2; // 返回退出码 2
        } // 分支结束
    }; // match 结束

    // 1. 对象枚举
    let objs = eng.objects(); // 获取所有数据库对象
    let has = |n: &str, k: ObjKind| objs.iter().any(|o| o.name == n && o.kind == k); // 判断是否存在指定对象
    let n_tables = objs.iter().filter(|o| o.kind == ObjKind::Table).count(); // 统计表数量
    check( // 用例 1：对象枚举
        "objects/表枚举", // 用例名
        n_tables >= 4 && has("customers", ObjKind::Table) && has("v_order_summary", ObjKind::View), // 断言条件
        format!("{n_tables} tables, {} views", objs.iter().filter(|o| o.kind == ObjKind::View).count()), // 附加详情
    ); // check 结束

    // 2. 行数统计
    let cnt = eng.count("customers"); // 统计 customers 行数
    check("count/行数统计", cnt == 6, format!("customers = {cnt}")); // 用例 2：期望 6 行

    // 3. 查询结果形状
    let q = eng.query("SELECT id, name FROM customers ORDER BY id"); // 执行查询
    check( // 用例 3：结果集
        "query/结果集", // 用例名
        q.error.is_none() && q.columns.len() == 2 && q.rows.len() == 6, // 断言：无错误、2 列、6 行
        format!("cols={:?} rows={}", q.columns, q.rows.len()), // 附加详情
    ); // check 结束

    // 4. 插入 / 删除 往返
    let new_id = eng.insert_row( // 插入一行
        "customers", // 目标表
        &["id".into(), "name".into(), "email".into(), "country".into(), "created_at".into()], // 列名
        &[Some("99".into()), Some("Selftest".into()), Some("t@e.st".into()), Some("CN".into()), Some("2026-11-01".into())], // 值
    ); // insert_row 结束
    let after_ins = eng.count("customers"); // 插入后行数
    let rowid = match new_id { // 取插入行 id
        Ok(_) => 99i64, // 成功则 id 为 99
        Err(_) => -1, // 失败用 -1
    }; // match 结束
    let ins_ok = after_ins == 7; // 插入是否使行数变为 7
    let _ = eng.delete_row("customers", rowid); // 删除刚插入的行
    let after_del = eng.count("customers"); // 删除后行数
    check( // 用例 4：插入与删除
        "DML/插入与删除", // 用例名
        ins_ok && after_del == 6, // 断言：先到 7 再回到 6
        format!("insert -> {after_ins}, delete -> {after_del}"), // 附加详情
    ); // check 结束

    // 5. 单元格更新
    eng.update_cell("customers", 1, "country", Some("TESTLAND")).ok(); // 把 id=1 的 country 改为 TESTLAND
    let u = eng.query("SELECT country FROM customers WHERE id = 1"); // 查询验证
    let cell = u.rows.first().and_then(|r| r.first()).and_then(|c| c.clone()); // 取出单元格值
    check("DML/单元格更新", cell.as_deref() == Some("TESTLAND"), format!("country = {cell:?}")); // 用例 5
    eng.update_cell("customers", 1, "country", Some("China")).ok(); // 复原为 China

    // 6. 多语句脚本
    let res = eng.run_script("SELECT 1; SELECT 2;"); // 运行两条语句
    check("script/多语句", res.len() == 2 && res.iter().all(|r| r.error.is_none()), format!("{} results", res.len())); // 用例 6

    // 7. split_sql
    let parts = split_sql("SELECT 1; SELECT 'a;b'; -- note\nSELECT 3"); // 按分号切分（须忽略字符串内与注释内分号）
    check("split_sql/分号解析", parts.len() == 3, format!("{parts:?}")); // 用例 7：应为 3 段

    // 8. build_create_table + 执行 DDL
    let cols = vec![ // 构造两列定义
        Column { cid: 0, name: "id".into(), decl_type: "INTEGER".into(), notnull: false, default: None, pk: true }, // 主键 id
        Column { cid: 1, name: "label".into(), decl_type: "TEXT".into(), notnull: true, default: None, pk: false }, // 非空 label
    ]; // vec 结束
    let ddl_ok = build_create_table(&cols).map(|d| eng.execute_batch(&format!("CREATE TABLE t_new ({d})"))); // 生成并执行建表
    let created = ddl_ok.map(|r| r.is_ok()).unwrap_or(false) && eng.objects().iter().any(|o| o.name == "t_new"); // 是否创建成功
    check("DDL/建表语句", created, "CREATE TABLE t_new generated & executed".into()); // 用例 8

    // 9. 外键
    let fks = eng.foreign_keys(); // 查询外键
    check("外键/关系", !fks.is_empty(), format!("{} foreign keys", fks.len())); // 用例 9

    // 10. 完整性检查
    let integ = eng.integrity_check(); // 运行 PRAGMA integrity_check
    check("完整性检查", integ.to_lowercase().contains("ok"), integ); // 用例 10

    // 11. 版本
    let ver = eng.sqlite_version(); // 查询 SQLite 版本
    check("SQLite 版本", !ver.is_empty(), ver); // 用例 11

    // 12. 美化 SQL 须大写关键字但保留字符串字面量
    let b = crate::ui::query::beautify_sql("select name from customers where country = 'China'"); // 美化一段 SQL
    check( // 用例 12
        "美化SQL/非破坏性", // 用例名
        b.contains("SELECT") && b.contains("'China'") && !b.contains("'CHINA'"), // 断言：关键字大写、字符串不被改动
        b.replace('\n', " / "), // 附加详情（把换行替换为分隔符）
    ); // check 结束

    // 13. 数据脱敏变换
    use crate::ui::tools::mask_value; // 局部引入脱敏函数
    let m1 = mask_value("Alice", "姓名保留首字"); // 姓名规则
    let m2 = mask_value("alice@example.com", "邮箱掩码"); // 邮箱规则
    let m3 = mask_value("13800138000", "手机掩码"); // 手机规则
    let m4 = mask_value("secret", "全遮蔽 (***)"); // 全遮蔽规则
    check( // 用例 13
        "数据脱敏/规则", // 用例名
        m1 == "A**" && m2 == "a***@example.com" && m3 == "138****00" && m4 == "***", // 断言四种规则输出
        format!("name={m1} email={m2} phone={m3} all={m4}"), // 附加详情
    ); // check 结束

    // 14. “新增记录”占位策略须满足 NOT NULL 约束
    let before = eng.count("customers"); // 记录操作前行数
    let ins = eng.insert_row( // 仅提供部分列进行插入
        "customers", // 目标表
        &["name".into(), "email".into(), "country".into(), "created_at".into()], // 列名
        &[Some(String::new()), None, None, None], // 部分值为空
    ); // insert_row 结束
    let after = eng.count("customers"); // 操作后行数
    if let Ok(id) = &ins { let _ = eng.delete_row("customers", *id); } // 若成功则删除该行
    check( // 用例 14
        "添加记录/NOT NULL 占位", // 用例名
        ins.is_ok() && after == before + 1, // 断言：插入成功且净增 1 行
        format!("insert_ok={} rows {} -> {}", ins.is_ok(), before, after), // 附加详情
    ); // check 结束

    // 15. 结构对象：索引创建/删除
    let idx_ok = eng.create_index("idx_cust_country", "customers", &["country".into()], false).is_ok() // 创建索引是否成功
        && eng.indexes("customers").iter().any(|(n, _)| n == "idx_cust_country"); // 且索引已存在
    let drop_ok = eng.drop_object("INDEX", "idx_cust_country").is_ok() // 删除索引是否成功
        && !eng.indexes("customers").iter().any(|(n, _)| n == "idx_cust_country"); // 且索引已消失
    check("对象/索引增删", idx_ok && drop_ok, format!("create={idx_ok} drop={drop_ok}")); // 用例 15

    // 16. 视图创建/删除
    let v_ok = eng.create_view("v_test", "SELECT id FROM customers").is_ok() // 创建视图是否成功
        && eng.views_list().iter().any(|(n, _)| n == "v_test"); // 且视图已存在
    let v_drop = eng.drop_object("VIEW", "v_test").is_ok(); // 删除视图是否成功
    check("对象/视图增删", v_ok && v_drop, format!("create={v_ok} drop={v_drop}")); // 用例 16

    // 17. SQL 转储须包含结构与数据
    let dump = eng.dump_sql(None); // 导出整库 SQL
    check( // 用例 17
        "数据泵/SQL 导出", // 用例名
        dump.contains("CREATE TABLE") && dump.contains("INSERT INTO \"customers\"") && dump.contains("COMMIT"), // 断言内容
        format!("{} bytes", dump.len()), // 附加详情
    ); // check 结束

    // 18. 内存库备份（VACUUM INTO）
    let bak = std::env::temp_dir().join("navicat_rs_selftest_backup.sqlite"); // 备份文件路径
    let bak_s = bak.to_string_lossy().to_string(); // 转为字符串
    let bak_ok = eng.vacuum_into(&bak_s).is_ok() && std::fs::metadata(&bak).map(|m| m.len() > 0).unwrap_or(false); // 备份成功且文件非空
    let _ = std::fs::remove_file(&bak); // 删除备份文件
    check("备份/内存库 VACUUM INTO", bak_ok, format!("{bak_s}")); // 用例 18

    // 19. SQL 大纲 / 重构分析
    let report = crate::ui::tools::analyze_sql("SELECT * FROM customers; UPDATE customers SET country='x'; DELETE FROM products;"); // 分析一段 SQL
    let outline_ok = report.contains("customers") // 含表名
        && report.contains("SELECT *") // 含 SELECT *
        && report.contains("UPDATE 无 WHERE") // 提示 UPDATE 无 WHERE
        && report.contains("DELETE 无 WHERE"); // 提示 DELETE 无 WHERE
    check("SQL大纲/重构建议", outline_ok, format!("{} 字符", report.len())); // 用例 19

    // 20. 查找于数据库（跨表值搜索）
    let hits = crate::ui::tools::find_in_db(&eng, "China"); // 搜索值 China
    check("查找于数据库", !hits.is_empty() && hits.iter().any(|(t, _, _)| t == "customers"), format!("{} 处匹配", hits.len())); // 用例 20

    // 21. JSON 导入（对象数组，键取并集）
    let _ = eng.execute_batch("CREATE TABLE jt(id INTEGER, name TEXT)"); // 准备目标表
    let jr = crate::ui::tools::import_json_text(&eng, "jt", r#"[{"id":1,"name":"a"},{"id":2,"name":"b"}]"#); // 导入 JSON 数组
    let jcnt = eng.count("jt"); // 统计导入行数
    check("导入/JSON 数组", jr.map(|n| n == 2).unwrap_or(false) && jcnt == 2, format!("imported rows={jcnt}")); // 用例 21

    // 22. 跨引擎：真实 PostgreSQL 网络协议驱动已接入
    check( // 用例 22
        "跨引擎/真实驱动", // 用例名
        crate::realdb::has_real_driver(model::DbKind::PostgreSql), // 断言存在真实驱动
        "PostgreSQL 网络协议驱动已接入".into(), // 附加说明
    ); // check 结束

    println!("\n==== SELFTEST: {} passed, {} failed ====", pass, fail); // 输出汇总
    if fail == 0 { 0 } else { 1 } // 全部通过返回 0，否则返回 1
} // selftest 结束

/// 程序主入口：解析 `--selftest`，否则创建窗口并运行 egui 应用。
fn main() -> eframe::Result<()> { // 返回 eframe 运行结果
    if std::env::args().any(|a| a == "--selftest") { // 若命令行含 --selftest
        std::process::exit(selftest()); // 运行自检并以其结果退出
    } // if 结束
    let options = eframe::NativeOptions { // 配置原生窗口
        viewport: egui::ViewportBuilder::default() // 默认视口构建器
            .with_inner_size([1440.0, 900.0]) // 初始窗口尺寸
            .with_min_inner_size([1000.0, 640.0]) // 最小窗口尺寸
            .with_title("navicat-rs"), // 窗口标题
        ..Default::default() // 其它选项默认
    }; // NativeOptions 结束
    eframe::run_native( // 启动原生应用
        "navicat-rs", // 应用名
        options, // 窗口选项
        Box::new(|cc| Ok(Box::new(app::NavicatApp::new(cc)))), // 创建应用实例的工厂闭包
    ) // run_native 调用结束
} // main 结束
