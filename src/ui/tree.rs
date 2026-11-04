use crate::app::NavicatApp; // 引入应用对象
use crate::model::ObjKind; // 引入对象类型
use egui::{RichText, Stroke}; // 引入 egui 类型

enum Pending { // 待处理动作枚举
    Open(String), // 打开连接
    Edit(String), // 编辑连接
    Delete(String), // 删除连接
    Toggle(String), // 切换展开/折叠
    Table(String, String), // 打开数据表
    Design(String, String), // 打开设计器
    Profiling(String, String), // 打开数据剖析
    Datagen(String, String), // 打开数据生成
    Masking(String, String), // 打开数据脱敏
    Dict(String, String), // 打开数据字典
    Export(String, String), // 导出
    DropTable(String, String), // 删除表
    Favorite(String, String), // 收藏表
    Unfavorite(String, String), // 取消收藏
} // 枚举结束

pub fn draw(app: &mut NavicatApp, ctx: &egui::Context) { // 绘制对象树面板
    egui::SidePanel::left("tree") // 左侧面板
        .resizable(true) // 允许调整宽度
        .default_width(app.left_w) // 默认宽度
        .width_range(200.0..=520.0) // 宽度范围
        .frame(egui::Frame::none().fill(app.pal.panel_alt).inner_margin(egui::Margin::same(6.0))) // 面板样式
        .show(ctx, |ui| { // 开始绘制
            let mut zoom = app.store.settings.tree_zoom.clamp(0.7, 1.9); // 树缩放系数（限幅 0.7..1.9）
            let mut new_zoom: Option<f32> = None; // 新的缩放值
            ui.horizontal(|ui| { // 横向布局
                ui.label(RichText::new("连接").size(13.0 * zoom).strong()); // “连接”标题
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { // 右对齐布局
                    if ui.small_button("\u{2795}").on_hover_text("新建连接").clicked() { // 新建连接按钮
                        app.new_connection(crate::model::DbKind::MySql); // 以 MySQL 新建连接
                    } // if 结束
                    if ui.small_button("\u{FF0B}").on_hover_text("放大树视图 (Ctrl+滚轮)").clicked() { // 放大按钮
                        new_zoom = Some((zoom + 0.1).min(1.9)); // 增大缩放值
                    } // if 结束
                    if ui.small_button("\u{FF0D}").on_hover_text("缩小树视图 (Ctrl+滚轮)").clicked() { // 缩小按钮
                        new_zoom = Some((zoom - 0.1).max(0.7)); // 减小缩放值
                    } // if 结束
                    ui.label(RichText::new(format!("{:.0}%", zoom * 100.0)).size(11.0).color(app.pal.text_dim)) // 缩放百分比文字
                        .on_hover_text("树形缩放（双击此处复位）"); // 悬停提示
                }); // 右对齐结束
            }); // 横向结束
            // Ctrl+mouse-wheel over the tree panel zooms the tree only.
            if ui.rect_contains_pointer(ui.max_rect()) { // 若指针在面板内
                let (ctrl, scroll) = ui.input(|i| (i.modifiers.ctrl, i.raw_scroll_delta.y)); // 读取 Ctrl 与滚轮增量
                if ctrl && scroll.abs() > 0.01 { // 若按住 Ctrl 且有滚动
                    new_zoom = Some((zoom + scroll * 0.0012).clamp(0.7, 1.9)); // 按滚轮调整缩放
                } // if 结束
            } // if 结束
            if let Some(z) = new_zoom { // 若有新的缩放值
                let z = (z * 10.0).round() / 10.0; // 取整到 0.1
                app.store.settings.tree_zoom = z; // 写回设置
                app.store.save(); // 保存设置
                zoom = z; // 更新局部缩放
            } // if 结束
            ui.add(egui::TextEdit::singleline(&mut app.tree_search).hint_text("\u{1F50D} 搜索连接/对象").desired_width(f32::INFINITY)); // 搜索框
            ui.add_space(4.0); // 间距

            let mut pending: Option<Pending> = None; // 本帧待处理动作
            let search = app.tree_search.to_lowercase(); // 搜索词（转小写）
            egui::ScrollArea::vertical().id_source("treescroll").auto_shrink([false, false]).show(ui, |ui| { // 纵向滚动区
                // Favorites section
                let favs: Vec<(String, String)> = app.store.favorites.iter() // 收藏列表
                    .filter_map(|k| k.split_once("||").map(|(a, b)| (a.to_string(), b.to_string()))) // 按 || 拆出连接 id 与对象名
                    .collect(); // 收集
                if !favs.is_empty() { // 若有收藏
                    ui.label(RichText::new(format!("\u{2605} 收藏 ({})", favs.len())).size(12.0 * zoom).color(app.pal.warn)); // 收藏分组标题
                    for (cid, t) in &favs { // 遍历收藏项
                        let resp = ui.add(egui::Label::new(RichText::new(format!("\u{2605} {}", t)).size(12.5 * zoom).color(app.pal.text)).sense(egui::Sense::click())); // 收藏项可点击标签
                        if resp.clicked() { pending = Some(Pending::Table(cid.clone(), t.clone())); } // 点击打开该表
                        let c2 = cid.clone(); let t2 = t.clone(); // 克隆参数供闭包
                        resp.context_menu(|ui| { // 右键菜单
                            if ui.button("打开数据").clicked() { pending = Some(Pending::Table(c2.clone(), t2.clone())); ui.close_menu(); } // 打开数据
                            if ui.button("取消收藏").clicked() { pending = Some(Pending::Unfavorite(c2.clone(), t2.clone())); ui.close_menu(); } // 取消收藏
                        }); // 关闭菜单
                    } // for 结束
                    ui.separator(); // 分隔线
                } // if 结束
                let conns: Vec<(String, String, crate::model::DbKind)> = // 连接列表
                    app.store.connections.iter().map(|c| (c.id.clone(), c.name.clone(), c.kind)).collect(); // 收集连接信息
                if conns.is_empty() { // 若无连接
                    ui.add_space(10.0); // 间距
                    ui.label(RichText::new("尚无连接").color(app.pal.text_dim)); // 空态提示
                    ui.label(RichText::new("点击右上角 + 新建").size(11.0 * zoom).color(app.pal.text_dim)); // 新建提示
                } // if 结束
                for (id, name, kind) in conns { // 遍历连接
                    let is_open = app.engines.contains_key(&id); // 连接是否已打开
                    let expanded = *app.expanded.get(&id).unwrap_or(&false); // 是否处于展开状态
                    let selected = app.selected_conn.as_deref() == Some(id.as_str()); // 是否为选中连接
                    let bg = if selected { app.pal.selection } else { egui::Color32::TRANSPARENT }; // 行背景色
                    egui::Frame::none().fill(bg).rounding(egui::Rounding::same(5.0)).inner_margin(egui::Margin::symmetric(4.0, 2.0)).show(ui, |ui| { // 连接行框架
                        ui.horizontal(|ui| { // 横向布局
                            let arrow = if expanded { "\u{25BC}" } else { "\u{25B6}" }; // 展开/折叠箭头
                            if ui.small_button(arrow).clicked() { // 箭头按钮
                                if is_open { pending = Some(Pending::Toggle(id.clone())); } // 已打开则切换展开
                                else { pending = Some(Pending::Open(id.clone())); } // 否则打开连接
                            } // if 结束
                            let dot = RichText::new("\u{25CF}").color(kind.color()); // 引擎品牌色圆点
                            ui.label(dot); // 圆点标签
                            let label = ui.add(egui::Label::new(RichText::new(&name).size(13.0 * zoom).color(app.pal.text)).sense(egui::Sense::click())); // 连接名可点击标签
                            if label.clicked() { // 单击时
                                pending = Some(Pending::Open(id.clone())); // 打开连接
                            } // if 结束
                            if label.double_clicked() { // 双击时
                                pending = Some(Pending::Edit(id.clone())); // 编辑连接
                            } // if 结束
                            label.context_menu(|ui| { // 连接右键菜单
                                if ui.button("打开连接").clicked() { pending = Some(Pending::Open(id.clone())); ui.close_menu(); } // 打开连接
                                if ui.button("编辑连接…").clicked() { pending = Some(Pending::Edit(id.clone())); ui.close_menu(); } // 编辑连接
                                if ui.button("删除连接").clicked() { pending = Some(Pending::Delete(id.clone())); ui.close_menu(); } // 删除连接
                            }); // 菜单结束
                        }); // 横向结束
                    }); // 框架结束

                    if is_open && expanded { // 若已打开且展开
                        let objs = app.objects_for(&id); // 取该连接的对象列表
                        let tables: Vec<String> = objs.iter().filter(|o| o.kind == ObjKind::Table && (search.is_empty() || o.name.to_lowercase().contains(&search))).map(|o| o.name.clone()).collect(); // 过滤出表（按搜索词）
                        let views: Vec<String> = objs.iter().filter(|o| o.kind == ObjKind::View && (search.is_empty() || o.name.to_lowercase().contains(&search))).map(|o| o.name.clone()).collect(); // 过滤出视图（按搜索词）
                        ui.indent(format!("obj{}", id), |ui| { // 缩进子区域
                            // 分组标题用 CollapsingHeader：自带绘制三角，点击只折叠/展开「本组」。
                            // （此前用的 ▾/▸ 字符本机字体缺失，会显示成空心方块，故改用绘制型折叠头。）
                            if !tables.is_empty() { // 若有表
                                egui::CollapsingHeader::new( // 表分组折叠头
                                    RichText::new(format!("\u{25A6} 表 ({})", tables.len())) // 表分组标题
                                        .size(12.0 * zoom) // 字号
                                        .color(app.pal.text_dim), // 颜色
                                ) // 构造结束
                                .id_source(format!("{}::grp::tables", id)) // 分组 id
                                .default_open(true) // 默认展开
                                .show(ui, |ui| { // 绘制
                                    for t in &tables { // 遍历表
                                        tree_item(ui, app, &id, t, "\u{25A6}", zoom, &mut pending); // 绘制表项
                                    } // for 结束
                                }); // 折叠头结束
                            } // if 结束
                            if !views.is_empty() { // 若有视图
                                egui::CollapsingHeader::new( // 视图分组折叠头
                                    RichText::new(format!("\u{25C9} 视图 ({})", views.len())) // 视图分组标题
                                        .size(12.0 * zoom) // 字号
                                        .color(app.pal.text_dim), // 颜色
                                ) // 构造结束
                                .id_source(format!("{}::grp::views", id)) // 分组 id
                                .default_open(true) // 默认展开
                                .show(ui, |ui| { // 绘制
                                    for v in &views { // 遍历视图
                                        tree_item(ui, app, &id, v, "\u{25C9}", zoom, &mut pending); // 绘制视图项
                                    } // for 结束
                                }); // 折叠头结束
                            } // if 结束
                            // Feature folders (conceptual, mirror Navicat tree) — 各自可折叠。
                            for (fkey, flabel) in [("backup", "备份"), ("model", "模型"), ("users", "用户")] { // 遍历功能分组（备份/模型/用户）
                                egui::CollapsingHeader::new( // 功能分组折叠头
                                    RichText::new(format!("\u{25A4} {}", flabel)) // 分组标题
                                        .size(12.0 * zoom) // 字号
                                        .color(app.pal.text_dim), // 颜色
                                ) // 构造结束
                                .id_source(format!("{}::grp::{}", id, fkey)) // 分组 id
                                .default_open(false) // 默认收起
                                .show(ui, |ui| { // 绘制
                                    ui.label( // 空内容标签
                                        RichText::new("（暂无内容）") // 暂无内容
                                            .size(11.0 * zoom) // 字号
                                            .color(app.pal.text_dim), // 颜色
                                    ); // 构造结束
                                }); // 绘制结束
                            } // 折叠头结束
                        }); // 缩进区结束
                    } // if 结束
                } // for 结束
            }); // 滚动区结束

            if let Some(p) = pending { // 若有待处理动作
                match p { // 匹配动作
                    Pending::Open(id) => { // 打开连接
                        if app.engines.contains_key(&id) { // 若已打开
                            let cur = *app.expanded.get(&id).unwrap_or(&false); // 当前展开状态
                            app.expanded.insert(id.clone(), !cur); // 切换展开
                            app.selected_conn = Some(id); // 设为选中连接
                        } else { app.open_connection(id); } // 否则打开连接
                    } // if 结束
                    Pending::Toggle(id) => { // 切换展开
                        let cur = *app.expanded.get(&id).unwrap_or(&false); // 当前展开状态
                        app.expanded.insert(id, !cur); // 取反写回
                    } // 分支结束
                    Pending::Edit(id) => app.edit_connection(id), // 编辑连接
                    Pending::Delete(id) => app.request_delete_connection(id), // 删除连接
                    Pending::Table(id, t) => app.open_table(id, t), // 打开表
                    Pending::Design(id, t) => app.open_designer(id, Some(t)), // 打开设计器
                    Pending::Profiling(id, t) => app.open_profiling(id, t), // 打开数据剖析
                    Pending::Datagen(id, t) => app.open_datagen(id, t), // 打开数据生成
                    Pending::Masking(id, t) => app.open_masking(id, t), // 打开数据脱敏
                    Pending::Dict(id, t) => app.open_dict(id, t), // 打开数据字典
                    Pending::Export(id, t) => app.open_export(id, t), // 打开导出
                    Pending::DropTable(id, t) => app.dialog = Some(crate::app::Dialog::Confirm(crate::app::ConfirmData { // 弹出删除表确认框
                        title: "删除表".into(), // 标题
                        msg: format!("确定删除表 “{}” 吗？", t), // 消息
                        action: crate::app::Act::DropTable(id, t), // 动作
                    })), // 构造结束
                    Pending::Favorite(id, t) => { // 收藏表
                        let key = format!("{}||{}", id, t); // 收藏键
                        if !app.store.favorites.contains(&key) { app.store.favorites.push(key); app.store.save(); } // 不存在则加入并保存
                    } // 分支结束
                    Pending::Unfavorite(id, t) => { // 取消收藏
                        let key = format!("{}||{}", id, t); // 收藏键
                        app.store.favorites.retain(|k| k != &key); app.store.save(); // 移除并保存
                    } // 分支结束
                } // match 结束
            } // if 结束
        }); // 面板绘制结束
} // draw 结束

fn tree_item(ui: &mut egui::Ui, app: &NavicatApp, conn_id: &str, name: &str, icon: &str, zoom: f32, pending: &mut Option<Pending>) { // 绘制单个树项
    let resp = ui.add(egui::Label::new(RichText::new(format!("{} {}", icon, name)).size(12.5 * zoom).color(app.pal.text)).sense(egui::Sense::click())); // 树项可点击标签
    if resp.clicked() { // 若被点击
        *pending = Some(Pending::Table(conn_id.to_string(), name.to_string())); // 打开该表
    } // if 结束
    let cid = conn_id.to_string(); // 连接 id 字符串
    let nm = name.to_string(); // 名称字符串
    let key = format!("{}||{}", conn_id, name); // 收藏键
    let is_fav = app.store.favorites.contains(&key); // 是否已收藏
    resp.context_menu(|ui| { // 右键菜单
        if ui.button("打开数据").clicked() { *pending = Some(Pending::Table(cid.clone(), nm.clone())); ui.close_menu(); } // 打开数据
        if ui.button("设计表").clicked() { *pending = Some(Pending::Design(cid.clone(), nm.clone())); ui.close_menu(); } // 设计表
        if ui.button("数据剖析").clicked() { *pending = Some(Pending::Profiling(cid.clone(), nm.clone())); ui.close_menu(); } // 数据剖析
        if ui.button("数据生成").clicked() { *pending = Some(Pending::Datagen(cid.clone(), nm.clone())); ui.close_menu(); } // 数据生成
        if ui.button("数据脱敏").clicked() { *pending = Some(Pending::Masking(cid.clone(), nm.clone())); ui.close_menu(); } // 数据脱敏
        if ui.button("数据字典").clicked() { *pending = Some(Pending::Dict(cid.clone(), nm.clone())); ui.close_menu(); } // 数据字典
        if ui.button("导出").clicked() { *pending = Some(Pending::Export(cid.clone(), nm.clone())); ui.close_menu(); } // 导出
        if is_fav { // 若已收藏
            if ui.button("\u{2605} 取消收藏").clicked() { *pending = Some(Pending::Unfavorite(cid.clone(), nm.clone())); ui.close_menu(); } // 取消收藏
        } else { // 否则
            if ui.button("\u{2606} 收藏此表").clicked() { *pending = Some(Pending::Favorite(cid.clone(), nm.clone())); ui.close_menu(); } // 收藏此表
        } // if 结束
        ui.separator(); // 分隔线
        if ui.button("删除表").clicked() { *pending = Some(Pending::DropTable(cid.clone(), nm.clone())); ui.close_menu(); } // 删除表
    }); // 菜单结束
    let _ = Stroke::NONE; // 忽略未使用变量
} // tree_item 结束
