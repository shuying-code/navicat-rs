#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod app;
mod model;
mod store;
mod db;
mod realdb;
mod theme;
mod ui;

/// Headless functional self-test of the SQLite engine layer.
/// Run with: navicat-rs.exe --selftest
fn selftest() -> i32 {
    use db::{build_create_table, split_sql, SqliteEngine};
    use model::{Column, ObjKind};

    let mut pass = 0u32;
    let mut fail = 0u32;
    let mut check = |name: &str, ok: bool, detail: String| {
        if ok {
            pass += 1;
            println!("[PASS] {name}  {detail}");
        } else {
            fail += 1;
            println!("[FAIL] {name}  {detail}");
        }
    };

    let eng = match SqliteEngine::open_memory_seeded() {
        Ok(e) => e,
        Err(e) => {
            println!("[FATAL] cannot open seeded engine: {e}");
            return 2;
        }
    };

    // 1. object enumeration
    let objs = eng.objects();
    let has = |n: &str, k: ObjKind| objs.iter().any(|o| o.name == n && o.kind == k);
    let n_tables = objs.iter().filter(|o| o.kind == ObjKind::Table).count();
    check(
        "objects/表枚举",
        n_tables >= 4 && has("customers", ObjKind::Table) && has("v_order_summary", ObjKind::View),
        format!("{n_tables} tables, {} views", objs.iter().filter(|o| o.kind == ObjKind::View).count()),
    );

    // 2. select count
    let cnt = eng.count("customers");
    check("count/行数统计", cnt == 6, format!("customers = {cnt}"));

    // 3. query result shape
    let q = eng.query("SELECT id, name FROM customers ORDER BY id");
    check(
        "query/结果集",
        q.error.is_none() && q.columns.len() == 2 && q.rows.len() == 6,
        format!("cols={:?} rows={}", q.columns, q.rows.len()),
    );

    // 4. insert / delete round trip
    let new_id = eng.insert_row(
        "customers",
        &["id".into(), "name".into(), "email".into(), "country".into(), "created_at".into()],
        &[Some("99".into()), Some("Selftest".into()), Some("t@e.st".into()), Some("CN".into()), Some("2026-11-01".into())],
    );
    let after_ins = eng.count("customers");
    let rowid = match new_id {
        Ok(_) => 99i64,
        Err(_) => -1,
    };
    let ins_ok = after_ins == 7;
    let _ = eng.delete_row("customers", rowid);
    let after_del = eng.count("customers");
    check(
        "DML/插入与删除",
        ins_ok && after_del == 6,
        format!("insert -> {after_ins}, delete -> {after_del}"),
    );

    // 5. update cell
    eng.update_cell("customers", 1, "country", Some("TESTLAND")).ok();
    let u = eng.query("SELECT country FROM customers WHERE id = 1");
    let cell = u.rows.first().and_then(|r| r.first()).and_then(|c| c.clone());
    check("DML/单元格更新", cell.as_deref() == Some("TESTLAND"), format!("country = {cell:?}"));
    eng.update_cell("customers", 1, "country", Some("China")).ok();

    // 6. multi-statement script
    let res = eng.run_script("SELECT 1; SELECT 2;");
    check("script/多语句", res.len() == 2 && res.iter().all(|r| r.error.is_none()), format!("{} results", res.len()));

    // 7. split_sql
    let parts = split_sql("SELECT 1; SELECT 'a;b'; -- note\nSELECT 3");
    check("split_sql/分号解析", parts.len() == 3, format!("{parts:?}"));

    // 8. build_create_table + execute DDL
    let cols = vec![
        Column { cid: 0, name: "id".into(), decl_type: "INTEGER".into(), notnull: false, default: None, pk: true },
        Column { cid: 1, name: "label".into(), decl_type: "TEXT".into(), notnull: true, default: None, pk: false },
    ];
    let ddl_ok = build_create_table(&cols).map(|d| eng.execute_batch(&format!("CREATE TABLE t_new ({d})")));
    let created = ddl_ok.map(|r| r.is_ok()).unwrap_or(false) && eng.objects().iter().any(|o| o.name == "t_new");
    check("DDL/建表语句", created, "CREATE TABLE t_new generated & executed".into());

    // 9. foreign keys
    let fks = eng.foreign_keys();
    check("外键/关系", !fks.is_empty(), format!("{} foreign keys", fks.len()));

    // 10. integrity
    let integ = eng.integrity_check();
    check("完整性检查", integ.to_lowercase().contains("ok"), integ);

    // 11. pragma / version
    let ver = eng.sqlite_version();
    check("SQLite 版本", !ver.is_empty(), ver);

    // 12. beautify must uppercase keywords but preserve string literals
    let b = crate::ui::query::beautify_sql("select name from customers where country = 'China'");
    check(
        "美化SQL/非破坏性",
        b.contains("SELECT") && b.contains("'China'") && !b.contains("'CHINA'"),
        b.replace('\n', " / "),
    );

    // 13. data masking transforms
    use crate::ui::tools::mask_value;
    let m1 = mask_value("Alice", "姓名保留首字");
    let m2 = mask_value("alice@example.com", "邮箱掩码");
    let m3 = mask_value("13800138000", "手机掩码");
    let m4 = mask_value("secret", "全遮蔽 (***)");
    check(
        "数据脱敏/规则",
        m1 == "A**" && m2 == "a***@example.com" && m3 == "138****00" && m4 == "***",
        format!("name={m1} email={m2} phone={m3} all={m4}"),
    );

    // 14. "add record" placeholder strategy must satisfy NOT NULL constraints
    let before = eng.count("customers");
    let ins = eng.insert_row(
        "customers",
        &["name".into(), "email".into(), "country".into(), "created_at".into()],
        &[Some(String::new()), None, None, None],
    );
    let after = eng.count("customers");
    if let Ok(id) = &ins { let _ = eng.delete_row("customers", *id); }
    check(
        "添加记录/NOT NULL 占位",
        ins.is_ok() && after == before + 1,
        format!("insert_ok={} rows {} -> {}", ins.is_ok(), before, after),
    );

    // 15. structure objects: index create/drop
    let idx_ok = eng.create_index("idx_cust_country", "customers", &["country".into()], false).is_ok()
        && eng.indexes("customers").iter().any(|(n, _)| n == "idx_cust_country");
    let drop_ok = eng.drop_object("INDEX", "idx_cust_country").is_ok()
        && !eng.indexes("customers").iter().any(|(n, _)| n == "idx_cust_country");
    check("对象/索引增删", idx_ok && drop_ok, format!("create={idx_ok} drop={drop_ok}"));

    // 16. view create/drop
    let v_ok = eng.create_view("v_test", "SELECT id FROM customers").is_ok()
        && eng.views_list().iter().any(|(n, _)| n == "v_test");
    let v_drop = eng.drop_object("VIEW", "v_test").is_ok();
    check("对象/视图增删", v_ok && v_drop, format!("create={v_ok} drop={v_drop}"));

    // 17. SQL dump contains schema + data
    let dump = eng.dump_sql(None);
    check(
        "数据泵/SQL 导出",
        dump.contains("CREATE TABLE") && dump.contains("INSERT INTO \"customers\"") && dump.contains("COMMIT"),
        format!("{} bytes", dump.len()),
    );

    // 18. memory DB backup via VACUUM INTO
    let bak = std::env::temp_dir().join("navicat_rs_selftest_backup.sqlite");
    let bak_s = bak.to_string_lossy().to_string();
    let bak_ok = eng.vacuum_into(&bak_s).is_ok() && std::fs::metadata(&bak).map(|m| m.len() > 0).unwrap_or(false);
    let _ = std::fs::remove_file(&bak);
    check("备份/内存库 VACUUM INTO", bak_ok, format!("{bak_s}"));

    // 19. SQL outline / refactor analysis
    let report = crate::ui::tools::analyze_sql("SELECT * FROM customers; UPDATE customers SET country='x'; DELETE FROM products;");
    let outline_ok = report.contains("customers")
        && report.contains("SELECT *")
        && report.contains("UPDATE 无 WHERE")
        && report.contains("DELETE 无 WHERE");
    check("SQL大纲/重构建议", outline_ok, format!("{} 字符", report.len()));

    // 20. find in database (cross-table value search)
    let hits = crate::ui::tools::find_in_db(&eng, "China");
    check("查找于数据库", !hits.is_empty() && hits.iter().any(|(t, _, _)| t == "customers"), format!("{} 处匹配", hits.len()));

    // 21. JSON import (array of objects, union of keys)
    let _ = eng.execute_batch("CREATE TABLE jt(id INTEGER, name TEXT)");
    let jr = crate::ui::tools::import_json_text(&eng, "jt", r#"[{"id":1,"name":"a"},{"id":2,"name":"b"}]"#);
    let jcnt = eng.count("jt");
    check("导入/JSON 数组", jr.map(|n| n == 2).unwrap_or(false) && jcnt == 2, format!("imported rows={jcnt}"));

    // 22. cross-engine: real PostgreSQL wire-protocol driver is wired in
    check(
        "跨引擎/真实驱动",
        crate::realdb::has_real_driver(model::DbKind::PostgreSql),
        "PostgreSQL 网络协议驱动已接入".into(),
    );

    println!("\n==== SELFTEST: {} passed, {} failed ====", pass, fail);
    if fail == 0 { 0 } else { 1 }
}

fn main() -> eframe::Result<()> {
    if std::env::args().any(|a| a == "--selftest") {
        std::process::exit(selftest());
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1440.0, 900.0])
            .with_min_inner_size([1000.0, 640.0])
            .with_title("navicat-rs"),
        ..Default::default()
    };
    eframe::run_native(
        "navicat-rs",
        options,
        Box::new(|cc| Ok(Box::new(app::NavicatApp::new(cc)))),
    )
}

