//! 主题与配色模块：定义多套调色板（浅色/深色/午夜蓝/森林/暖沙/石墨），
//! 并提供把调色板应用到 egui 全局样式的函数。

use egui::{Color32, Rounding, Stroke, Visuals}; // 引入 egui 颜色、圆角、描边与样式类型

/// 一套完整的界面调色板。
#[derive(Clone)] // 派生：可克隆
pub struct Palette { // 调色板结构体
    pub bg: Color32, // 页面背景色
    pub panel: Color32, // 面板底色
    pub panel_alt: Color32, // 面板交替色
    pub border: Color32, // 边框色
    pub text: Color32, // 正文文字色
    pub text_dim: Color32, // 次要文字色
    pub accent: Color32, // 强调色
    pub accent_soft: Color32, // 强调色的浅色调
    pub selection: Color32, // 选区背景色
    pub toolbar: Color32, // 工具栏底色
    pub tab_active: Color32, // 选中标签页底色
    pub tab_inactive: Color32, // 未选中标签页底色
    pub success: Color32, // 成功状态色
    pub warn: Color32, // 警告状态色
    pub danger: Color32, // 危险状态色
    pub grid_header: Color32, // 表格表头底色
    pub grid_alt: Color32, // 表格斑马纹底色
} // 结构体结束

impl Palette { // 为 Palette 实现辅助方法
    /// 依据背景色亮度选择对比度最高的前景色（近黑或纯白），
    /// 用于“实心按钮”上的文字，确保任意主题下按钮文字都清晰可读。
    pub fn contrast_text(&self, bg: Color32) -> Color32 { // 计算与给定背景对比最佳的文字色
        fn lin(c: u8) -> f32 { // 把 sRGB 单通道线性化（WCAG 相对亮度计算用）
            let s = c as f32 / 255.0; // 归一化到 0..1
            if s <= 0.03928 { // 低值区间
                s / 12.92 // 线性段
            } else { // 高值区间
                ((s + 0.055) / 1.055).powf(2.4) // 幂函数段
            } // if 结束
        } // lin 结束
        let lum = |c: Color32| -> f32 { // 计算颜色相对亮度
            0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b()) // 加权求和
        }; // 闭包结束
        let bg_l = lum(bg); // 背景亮度
        let white_l = 1.0_f32; // 白色亮度
        let dark = Color32::from_rgb(0x14, 0x16, 0x1A); // 近黑色文字候选
        let dark_l = lum(dark); // 近黑亮度
        let ratio_white = (white_l + 0.05) / (bg_l + 0.05); // 与白色的对比度
        let ratio_dark = (bg_l + 0.05) / (dark_l + 0.05); // 与近黑的对比度
        if ratio_white >= ratio_dark { // 若白色对比更高
            Color32::WHITE // 用白色
        } else { // 否则
            dark // 用近黑色
        } // if 结束
    } // contrast_text 结束
} // impl Palette 结束

/// 浅色主题调色板。
pub fn light() -> Palette { // 返回浅色调色板
    Palette { // 构造调色板
        bg: Color32::from_rgb(0xF3, 0xF4, 0xF6), // 页面背景
        panel: Color32::from_rgb(0xFF, 0xFF, 0xFF), // 面板底
        panel_alt: Color32::from_rgb(0xF7, 0xF8, 0xFA), // 面板交替
        border: Color32::from_rgb(0xD5, 0xD8, 0xDD), // 边框
        text: Color32::from_rgb(0x20, 0x24, 0x2A), // 正文
        text_dim: Color32::from_rgb(0x6A, 0x71, 0x7A), // 次要文字
        accent: Color32::from_rgb(0x22, 0x74, 0xE8), // 强调
        accent_soft: Color32::from_rgb(0xE3, 0xEE, 0xFD), // 强调浅色
        selection: Color32::from_rgb(0xD6, 0xE7, 0xFF), // 选区
        toolbar: Color32::from_rgb(0xFA, 0xFB, 0xFC), // 工具栏
        tab_active: Color32::from_rgb(0xFF, 0xFF, 0xFF), // 选中标签
        tab_inactive: Color32::from_rgb(0xEC, 0xEE, 0xF1), // 未选中标签
        success: Color32::from_rgb(0x1E, 0x9E, 0x5A), // 成功
        warn: Color32::from_rgb(0xC8, 0x8A, 0x14), // 警告
        danger: Color32::from_rgb(0xD0, 0x3A, 0x3A), // 危险
        grid_header: Color32::from_rgb(0xEE, 0xF1, 0xF5), // 表头
        grid_alt: Color32::from_rgb(0xFA, 0xFB, 0xFD), // 斑马纹
    } // 构造结束
} // light 结束

/// 深色主题调色板。
pub fn dark() -> Palette { // 返回深色调色板
    Palette { // 构造调色板
        bg: Color32::from_rgb(0x1E, 0x20, 0x24), // 页面背景
        panel: Color32::from_rgb(0x25, 0x28, 0x2D), // 面板底
        panel_alt: Color32::from_rgb(0x2B, 0x2E, 0x34), // 面板交替
        border: Color32::from_rgb(0x3A, 0x3E, 0x45), // 边框
        text: Color32::from_rgb(0xE6, 0xE8, 0xEB), // 正文
        text_dim: Color32::from_rgb(0x9A, 0xA1, 0xAB), // 次要文字
        accent: Color32::from_rgb(0x4C, 0x9A, 0xFF), // 强调
        accent_soft: Color32::from_rgb(0x2A, 0x35, 0x48), // 强调浅色
        selection: Color32::from_rgb(0x2F, 0x45, 0x66), // 选区
        toolbar: Color32::from_rgb(0x22, 0x25, 0x2A), // 工具栏
        tab_active: Color32::from_rgb(0x2E, 0x32, 0x38), // 选中标签
        tab_inactive: Color32::from_rgb(0x23, 0x26, 0x2B), // 未选中标签
        success: Color32::from_rgb(0x4C, 0xC3, 0x7E), // 成功
        warn: Color32::from_rgb(0xE0, 0xB0, 0x4A), // 警告
        danger: Color32::from_rgb(0xE8, 0x6A, 0x6A), // 危险
        grid_header: Color32::from_rgb(0x2C, 0x30, 0x36), // 表头
        grid_alt: Color32::from_rgb(0x28, 0x2B, 0x31), // 斑马纹
    } // 构造结束
} // dark 结束

// ----------------- 多主题支持 -----------------
/// 单个主题的元信息（key 用于持久化，label 用于界面显示，dark 表示明暗）。
#[derive(Clone, Copy)] // 派生：可克隆、可拷贝
pub struct ThemeInfo { // 主题元信息结构体
    pub key: &'static str, // 主题唯一 key
    pub label: &'static str, // 主题显示名
    pub dark: bool, // 是否为深色主题
} // 结构体结束

/// 可供用户选择的主题列表（key 用于持久化，label 用于界面显示）。
pub const THEMES: &[ThemeInfo] = &[ // 主题常量数组
    ThemeInfo { key: "light", label: "浅色", dark: false }, // 浅色
    ThemeInfo { key: "dark", label: "深色", dark: true }, // 深色
    ThemeInfo { key: "midnight", label: "午夜蓝", dark: true }, // 午夜蓝
    ThemeInfo { key: "forest", label: "森林", dark: true }, // 森林
    ThemeInfo { key: "sand", label: "暖沙", dark: false }, // 暖沙
    ThemeInfo { key: "graphite", label: "石墨", dark: true }, // 石墨
]; // 数组结束

/// 判断指定 key 的主题是否为深色。
pub fn is_dark(key: &str) -> bool { // 查询主题明暗属性
    THEMES.iter().find(|t| t.key == key).map(|t| t.dark).unwrap_or(false) // 找到则取其 dark，否则默认 false
} // is_dark 结束

/// 返回指定 key 的主题显示名。
pub fn label_of(key: &str) -> &'static str { // 查询主题显示名
    THEMES.iter().find(|t| t.key == key).map(|t| t.label).unwrap_or("浅色") // 找到则取其 label，否则默认“浅色”
} // label_of 结束

/// 午夜蓝主题调色板。
pub fn midnight() -> Palette { // 返回午夜蓝调色板
    Palette { // 构造调色板
        bg: Color32::from_rgb(0x0F, 0x17, 0x20), // 页面背景
        panel: Color32::from_rgb(0x14, 0x1C, 0x28), // 面板底
        panel_alt: Color32::from_rgb(0x19, 0x22, 0x2F), // 面板交替
        border: Color32::from_rgb(0x27, 0x31, 0x3F), // 边框
        text: Color32::from_rgb(0xE6, 0xED, 0xF3), // 正文
        text_dim: Color32::from_rgb(0x8F, 0xA0, 0xB3), // 次要文字
        accent: Color32::from_rgb(0x38, 0xBD, 0xF8), // 强调
        accent_soft: Color32::from_rgb(0x13, 0x30, 0x3A), // 强调浅色
        selection: Color32::from_rgb(0x1E, 0x4A, 0x5A), // 选区
        toolbar: Color32::from_rgb(0x12, 0x1A, 0x24), // 工具栏
        tab_active: Color32::from_rgb(0x1B, 0x25, 0x31), // 选中标签
        tab_inactive: Color32::from_rgb(0x15, 0x1D, 0x28), // 未选中标签
        success: Color32::from_rgb(0x3D, 0xDC, 0x97), // 成功
        warn: Color32::from_rgb(0xF0, 0xB4, 0x29), // 警告
        danger: Color32::from_rgb(0xFF, 0x6B, 0x6B), // 危险
        grid_header: Color32::from_rgb(0x18, 0x22, 0x30), // 表头
        grid_alt: Color32::from_rgb(0x15, 0x1E, 0x2A), // 斑马纹
    } // 构造结束
} // midnight 结束

/// 森林主题调色板。
pub fn forest() -> Palette { // 返回森林调色板
    Palette { // 构造调色板
        bg: Color32::from_rgb(0x12, 0x21, 0x1B), // 页面背景
        panel: Color32::from_rgb(0x17, 0x2A, 0x22), // 面板底
        panel_alt: Color32::from_rgb(0x1C, 0x33, 0x29), // 面板交替
        border: Color32::from_rgb(0x2C, 0x4A, 0x3B), // 边框
        text: Color32::from_rgb(0xE4, 0xF2, 0xE9), // 正文
        text_dim: Color32::from_rgb(0x93, 0xB3, 0xA1), // 次要文字
        accent: Color32::from_rgb(0x36, 0xC0, 0x8A), // 强调
        accent_soft: Color32::from_rgb(0x1D, 0x3A, 0x2E), // 强调浅色
        selection: Color32::from_rgb(0x24, 0x51, 0x3C), // 选区
        toolbar: Color32::from_rgb(0x14, 0x24, 0x19), // 工具栏
        tab_active: Color32::from_rgb(0x1D, 0x33, 0x29), // 选中标签
        tab_inactive: Color32::from_rgb(0x17, 0x2A, 0x22), // 未选中标签
        success: Color32::from_rgb(0x3D, 0xDC, 0x97), // 成功
        warn: Color32::from_rgb(0xE0, 0xB0, 0x4A), // 警告
        danger: Color32::from_rgb(0xE8, 0x6A, 0x6A), // 危险
        grid_header: Color32::from_rgb(0x1B, 0x31, 0x28), // 表头
        grid_alt: Color32::from_rgb(0x17, 0x2A, 0x21), // 斑马纹
    } // 构造结束
} // forest 结束

/// 暖沙主题调色板。
pub fn sand() -> Palette { // 返回暖沙调色板
    Palette { // 构造调色板
        bg: Color32::from_rgb(0xF6, 0xF1, 0xE7), // 页面背景
        panel: Color32::from_rgb(0xFF, 0xFD, 0xF9), // 面板底
        panel_alt: Color32::from_rgb(0xF3, 0xEC, 0xE0), // 面板交替
        border: Color32::from_rgb(0xD9, 0xCD, 0xBB), // 边框
        text: Color32::from_rgb(0x3A, 0x33, 0x2A), // 正文
        text_dim: Color32::from_rgb(0x7A, 0x6F, 0x5E), // 次要文字
        accent: Color32::from_rgb(0xC2, 0x70, 0x3D), // 强调
        accent_soft: Color32::from_rgb(0xF6, 0xE6, 0xD6), // 强调浅色
        selection: Color32::from_rgb(0xF0, 0xDA, 0xC4), // 选区
        toolbar: Color32::from_rgb(0xFB, 0xF7, 0xF0), // 工具栏
        tab_active: Color32::from_rgb(0xFF, 0xFF, 0xFF), // 选中标签
        tab_inactive: Color32::from_rgb(0xF0, 0xE9, 0xDD), // 未选中标签
        success: Color32::from_rgb(0x2E, 0x9E, 0x5B), // 成功
        warn: Color32::from_rgb(0xC8, 0x8A, 0x14), // 警告
        danger: Color32::from_rgb(0xC2, 0x3B, 0x2E), // 危险
        grid_header: Color32::from_rgb(0xF0, 0xE8, 0xDA), // 表头
        grid_alt: Color32::from_rgb(0xFB, 0xF6, 0xEE), // 斑马纹
    } // 构造结束
} // sand 结束

/// 石墨主题调色板。
pub fn graphite() -> Palette { // 返回石墨调色板
    Palette { // 构造调色板
        bg: Color32::from_rgb(0x0E, 0x0E, 0x10), // 页面背景
        panel: Color32::from_rgb(0x16, 0x16, 0x18), // 面板底
        panel_alt: Color32::from_rgb(0x20, 0x20, 0x24), // 面板交替
        border: Color32::from_rgb(0x35, 0x35, 0x3B), // 边框
        text: Color32::from_rgb(0xED, 0xED, 0xEF), // 正文
        text_dim: Color32::from_rgb(0x9A, 0x9A, 0xA2), // 次要文字
        accent: Color32::from_rgb(0x8A, 0xB4, 0xF8), // 强调
        accent_soft: Color32::from_rgb(0x1E, 0x2A, 0x3A), // 强调浅色
        selection: Color32::from_rgb(0x2A, 0x3A, 0x55), // 选区
        toolbar: Color32::from_rgb(0x12, 0x12, 0x14), // 工具栏
        tab_active: Color32::from_rgb(0x1C, 0x1C, 0x20), // 选中标签
        tab_inactive: Color32::from_rgb(0x16, 0x16, 0x18), // 未选中标签
        success: Color32::from_rgb(0x81, 0xC9, 0x95), // 成功
        warn: Color32::from_rgb(0xFD, 0xD6, 0x63), // 警告
        danger: Color32::from_rgb(0xF2, 0x8B, 0x82), // 危险
        grid_header: Color32::from_rgb(0x1E, 0x1E, 0x22), // 表头
        grid_alt: Color32::from_rgb(0x18, 0x18, 0x1C), // 斑马纹
    } // 构造结束
} // graphite 结束

/// 根据主题 key 返回对应调色板（未知 key 回退为浅色）。
pub fn palette(key: &str) -> Palette { // 主题 key → 调色板
    match key { // 匹配主题 key
        "dark" => dark(), // 深色
        "midnight" => midnight(), // 午夜蓝
        "forest" => forest(), // 森林
        "sand" => sand(), // 暖沙
        "graphite" => graphite(), // 石墨
        _ => light(), // 默认浅色
    } // match 结束
} // palette 结束

/// 把指定主题应用到 egui 全局上下文，并返回其调色板。
pub fn apply_theme(ctx: &egui::Context, key: &str) -> Palette { // 应用主题
    let p = palette(key); // 取调色板
    let dark_mode = is_dark(key); // 判断是否深色
    let mut visuals = if dark_mode { Visuals::dark() } else { Visuals::light() }; // 选择 egui 明/深底样式
    visuals.override_text_color = Some(p.text); // 覆盖全局文字色
    visuals.panel_fill = p.panel; // 面板填充色
    visuals.window_fill = p.panel; // 窗口填充色
    visuals.extreme_bg_color = p.bg; // 极端背景色
    visuals.faint_bg_color = p.panel_alt; // 淡背景色
    visuals.widgets.noninteractive.bg_fill = p.panel; // 非交互控件背景
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.text); // 非交互控件前景
    visuals.widgets.inactive.bg_fill = p.panel_alt; // 未激活控件背景
    visuals.widgets.inactive.weak_bg_fill = p.panel_alt; // 未激活控件弱背景
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, p.text); // 未激活控件前景
    visuals.widgets.hovered.bg_fill = p.accent_soft; // 悬停控件背景
    visuals.widgets.hovered.weak_bg_fill = p.accent_soft; // 悬停控件弱背景
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, p.text); // 悬停控件前景
    visuals.widgets.active.bg_fill = p.accent; // 激活控件背景
    visuals.widgets.active.weak_bg_fill = p.accent; // 激活控件弱背景
    // 注意：egui 中 RichText::strong() 的文字颜色取自 widgets.active.text_color()，
    // 因此这里用主题前景色而非纯白，避免“标题/加粗内容字”在浅色主题下看不见或深色主题下变暗。
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, p.text); // 激活控件前景
    visuals.selection.bg_fill = p.selection; // 选区背景
    visuals.selection.stroke = Stroke::new(1.0, p.accent); // 选区描边
    visuals.window_stroke = Stroke::new(1.0, p.border); // 窗口描边
    visuals.window_rounding = Rounding::same(8.0); // 窗口圆角
    visuals.menu_rounding = Rounding::same(6.0); // 菜单圆角
    // 同时写入“浅色/深色”两套样式，并显式设置当前主题，确保所有面板/控件都使用本主题的
    // 文字与背景色（内容字不因主题切换而变得难读）。
    let theme = if dark_mode { egui::ThemePreference::Dark } else { egui::ThemePreference::Light }; // 构造主题偏好
    ctx.set_theme(theme); // 设定当前主题
    let vis2 = visuals.clone(); // 克隆一份样式用于闭包
    ctx.all_styles_mut(move |style| { // 修改所有样式
        style.visuals = vis2.clone(); // 应用调色板
        style.spacing.item_spacing = egui::vec2(6.0, 5.0); // 控件间距
        style.spacing.button_padding = egui::vec2(8.0, 4.0); // 按钮内边距
        style.spacing.menu_margin = egui::Margin::same(6.0); // 菜单边距
    }); // all_styles_mut 结束
    p // 返回调色板
} // apply_theme 结束
