//! Real network-protocol connectivity for engines beyond SQLite.
//!
//! SQLite is handled natively by `rusqlite`. For PostgreSQL we speak the real
//! wire protocol via the pure-Rust `postgres` driver so a PostgreSQL connection
//! profile actually connects to a live server and executes real SQL. Engines
//! without a bundled driver continue to use the offline demo schema, but the
//! connection profile still records everything needed to add a driver later.

use crate::db::QueryResult;
use crate::model::{Connection, DbKind};
use std::time::Instant;

fn pg_config(conn: &Connection) -> String {
    let mut s = format!(
        "host={} port={} user={} dbname={} connect_timeout=6",
        if conn.host.trim().is_empty() { "localhost" } else { conn.host.trim() },
        if conn.port == 0 { 5432 } else { conn.port },
        conn.username,
        conn.database
    );
    if !conn.password.is_empty() {
        s.push_str(&format!(" password={}", conn.password));
    }
    s
}

/// Attempt a real connection and return the server version string.
pub fn pg_test(conn: &Connection) -> Result<String, String> {
    use postgres::{Client, NoTls};
    let mut client = Client::connect(&pg_config(conn), NoTls).map_err(|e| e.to_string())?;
    let row = client.query_one("SELECT version()", &[]).map_err(|e| e.to_string())?;
    Ok(row.get::<_, String>(0))
}

/// Execute one or more statements against a live PostgreSQL server and map the
/// results into the same `QueryResult` shape used by the SQLite engine.
pub fn pg_run(conn: &Connection, sql: &str) -> Vec<QueryResult> {
    use postgres::{Client, NoTls, SimpleQueryMessage};
    let mut client = match Client::connect(&pg_config(conn), NoTls) {
        Ok(c) => c,
        Err(e) => {
            return vec![QueryResult { error: Some(format!("连接失败: {e}")), statement: sql.to_string(), ..Default::default() }];
        }
    };
    let t0 = Instant::now();
    match client.simple_query(sql) {
        Ok(msgs) => {
            let mut results: Vec<QueryResult> = Vec::new();
            let mut cols: Vec<String> = Vec::new();
            let mut rows: Vec<Vec<Option<String>>> = Vec::new();
            for m in msgs {
                match m {
                    SimpleQueryMessage::Row(r) => {
                        if cols.is_empty() {
                            cols = r.columns().iter().map(|c| c.name().to_string()).collect();
                        }
                        let mut rec = Vec::with_capacity(cols.len());
                        for i in 0..cols.len() {
                            rec.push(r.get(i).map(|s| s.to_string()));
                        }
                        rows.push(rec);
                    }
                    SimpleQueryMessage::CommandComplete(n) => {
                        if !rows.is_empty() {
                            results.push(QueryResult {
                                columns: std::mem::take(&mut cols),
                                rows: std::mem::take(&mut rows),
                                is_select: true,
                                statement: sql.to_string(),
                                elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
                                ..Default::default()
                            });
                        }
                        results.push(QueryResult {
                            affected: n as i64,
                            statement: sql.to_string(),
                            elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
                            ..Default::default()
                        });
                    }
                    _ => {}
                }
            }
            if !rows.is_empty() {
                results.push(QueryResult {
                    columns: cols,
                    rows,
                    is_select: true,
                    statement: sql.to_string(),
                    elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0,
                    ..Default::default()
                });
            }
            if results.is_empty() {
                results.push(QueryResult { statement: sql.to_string(), elapsed_ms: t0.elapsed().as_secs_f64() * 1000.0, ..Default::default() });
            }
            results
        }
        Err(e) => vec![QueryResult { error: Some(e.to_string()), statement: sql.to_string(), ..Default::default() }],
    }
}

/// Whether a real network driver is implemented for this engine.
pub fn has_real_driver(kind: DbKind) -> bool {
    matches!(kind, DbKind::PostgreSql)
}
