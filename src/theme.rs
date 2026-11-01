use egui::{Color32, Rounding, Stroke, Visuals};

#[derive(Clone)]
pub struct Palette {
    pub bg: Color32,
    pub panel: Color32,
    pub panel_alt: Color32,
    pub border: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub accent: Color32,
    pub accent_soft: Color32,
    pub selection: Color32,
    pub toolbar: Color32,
    pub tab_active: Color32,
    pub tab_inactive: Color32,
    pub success: Color32,
    pub warn: Color32,
    pub danger: Color32,
    pub grid_header: Color32,
    pub grid_alt: Color32,
}

impl Palette {
    /// 依据背景色亮度选择对比度最高的前景色（近黑或纯白），
    /// 用于"实心按钮"上的文字，确保任意主题下按钮文字都清晰可读。
    pub fn contrast_text(&self, bg: Color32) -> Color32 {
        fn lin(c: u8) -> f32 {
            let s = c as f32 / 255.0;
            if s <= 0.03928 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        }
        let lum = |c: Color32| -> f32 {
            0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
        };
        let bg_l = lum(bg);
        let white_l = 1.0_f32;
        let dark = Color32::from_rgb(0x14, 0x16, 0x1A);
        let dark_l = lum(dark);
        let ratio_white = (white_l + 0.05) / (bg_l + 0.05);
        let ratio_dark = (bg_l + 0.05) / (dark_l + 0.05);
        if ratio_white >= ratio_dark {
            Color32::WHITE
        } else {
            dark
        }
    }
}

pub fn light() -> Palette {
    Palette {
        bg: Color32::from_rgb(0xF3, 0xF4, 0xF6),
        panel: Color32::from_rgb(0xFF, 0xFF, 0xFF),
        panel_alt: Color32::from_rgb(0xF7, 0xF8, 0xFA),
        border: Color32::from_rgb(0xD5, 0xD8, 0xDD),
        text: Color32::from_rgb(0x20, 0x24, 0x2A),
        text_dim: Color32::from_rgb(0x6A, 0x71, 0x7A),
        accent: Color32::from_rgb(0x22, 0x74, 0xE8),
        accent_soft: Color32::from_rgb(0xE3, 0xEE, 0xFD),
        selection: Color32::from_rgb(0xD6, 0xE7, 0xFF),
        toolbar: Color32::from_rgb(0xFA, 0xFB, 0xFC),
        tab_active: Color32::from_rgb(0xFF, 0xFF, 0xFF),
        tab_inactive: Color32::from_rgb(0xEC, 0xEE, 0xF1),
        success: Color32::from_rgb(0x1E, 0x9E, 0x5A),
        warn: Color32::from_rgb(0xC8, 0x8A, 0x14),
        danger: Color32::from_rgb(0xD0, 0x3A, 0x3A),
        grid_header: Color32::from_rgb(0xEE, 0xF1, 0xF5),
        grid_alt: Color32::from_rgb(0xFA, 0xFB, 0xFD),
    }
}

pub fn dark() -> Palette {
    Palette {
        bg: Color32::from_rgb(0x1E, 0x20, 0x24),
        panel: Color32::from_rgb(0x25, 0x28, 0x2D),
        panel_alt: Color32::from_rgb(0x2B, 0x2E, 0x34),
        border: Color32::from_rgb(0x3A, 0x3E, 0x45),
        text: Color32::from_rgb(0xE6, 0xE8, 0xEB),
        text_dim: Color32::from_rgb(0x9A, 0xA1, 0xAB),
        accent: Color32::from_rgb(0x4C, 0x9A, 0xFF),
        accent_soft: Color32::from_rgb(0x2A, 0x35, 0x48),
        selection: Color32::from_rgb(0x2F, 0x45, 0x66),
        toolbar: Color32::from_rgb(0x22, 0x25, 0x2A),
        tab_active: Color32::from_rgb(0x2E, 0x32, 0x38),
        tab_inactive: Color32::from_rgb(0x23, 0x26, 0x2B),
        success: Color32::from_rgb(0x4C, 0xC3, 0x7E),
        warn: Color32::from_rgb(0xE0, 0xB0, 0x4A),
        danger: Color32::from_rgb(0xE8, 0x6A, 0x6A),
        grid_header: Color32::from_rgb(0x2C, 0x30, 0x36),
        grid_alt: Color32::from_rgb(0x28, 0x2B, 0x31),
    }
}

// ----------------- 多主题支持 -----------------
#[derive(Clone, Copy)]
pub struct ThemeInfo {
    pub key: &'static str,
    pub label: &'static str,
    pub dark: bool,
}

/// 可供用户选择的主题列表（key 用于持久化，label 用于界面显示）。
pub const THEMES: &[ThemeInfo] = &[
    ThemeInfo { key: "light", label: "浅色", dark: false },
    ThemeInfo { key: "dark", label: "深色", dark: true },
    ThemeInfo { key: "midnight", label: "午夜蓝", dark: true },
    ThemeInfo { key: "forest", label: "森林", dark: true },
    ThemeInfo { key: "sand", label: "暖沙", dark: false },
    ThemeInfo { key: "graphite", label: "石墨", dark: true },
];

pub fn is_dark(key: &str) -> bool {
    THEMES.iter().find(|t| t.key == key).map(|t| t.dark).unwrap_or(false)
}

pub fn label_of(key: &str) -> &'static str {
    THEMES.iter().find(|t| t.key == key).map(|t| t.label).unwrap_or("浅色")
}

pub fn midnight() -> Palette {
    Palette {
        bg: Color32::from_rgb(0x0F, 0x17, 0x20),
        panel: Color32::from_rgb(0x14, 0x1C, 0x28),
        panel_alt: Color32::from_rgb(0x19, 0x22, 0x2F),
        border: Color32::from_rgb(0x27, 0x31, 0x3F),
        text: Color32::from_rgb(0xE6, 0xED, 0xF3),
        text_dim: Color32::from_rgb(0x8F, 0xA0, 0xB3),
        accent: Color32::from_rgb(0x38, 0xBD, 0xF8),
        accent_soft: Color32::from_rgb(0x13, 0x30, 0x3A),
        selection: Color32::from_rgb(0x1E, 0x4A, 0x5A),
        toolbar: Color32::from_rgb(0x12, 0x1A, 0x24),
        tab_active: Color32::from_rgb(0x1B, 0x25, 0x31),
        tab_inactive: Color32::from_rgb(0x15, 0x1D, 0x28),
        success: Color32::from_rgb(0x3D, 0xDC, 0x97),
        warn: Color32::from_rgb(0xF0, 0xB4, 0x29),
        danger: Color32::from_rgb(0xFF, 0x6B, 0x6B),
        grid_header: Color32::from_rgb(0x18, 0x22, 0x30),
        grid_alt: Color32::from_rgb(0x15, 0x1E, 0x2A),
    }
}

pub fn forest() -> Palette {
    Palette {
        bg: Color32::from_rgb(0x12, 0x21, 0x1B),
        panel: Color32::from_rgb(0x17, 0x2A, 0x22),
        panel_alt: Color32::from_rgb(0x1C, 0x33, 0x29),
        border: Color32::from_rgb(0x2C, 0x4A, 0x3B),
        text: Color32::from_rgb(0xE4, 0xF2, 0xE9),
        text_dim: Color32::from_rgb(0x93, 0xB3, 0xA1),
        accent: Color32::from_rgb(0x36, 0xC0, 0x8A),
        accent_soft: Color32::from_rgb(0x1D, 0x3A, 0x2E),
        selection: Color32::from_rgb(0x24, 0x51, 0x3C),
        toolbar: Color32::from_rgb(0x14, 0x24, 0x19),
        tab_active: Color32::from_rgb(0x1D, 0x33, 0x29),
        tab_inactive: Color32::from_rgb(0x17, 0x2A, 0x22),
        success: Color32::from_rgb(0x3D, 0xDC, 0x97),
        warn: Color32::from_rgb(0xE0, 0xB0, 0x4A),
        danger: Color32::from_rgb(0xE8, 0x6A, 0x6A),
        grid_header: Color32::from_rgb(0x1B, 0x31, 0x28),
        grid_alt: Color32::from_rgb(0x17, 0x2A, 0x21),
    }
}

pub fn sand() -> Palette {
    Palette {
        bg: Color32::from_rgb(0xF6, 0xF1, 0xE7),
        panel: Color32::from_rgb(0xFF, 0xFD, 0xF9),
        panel_alt: Color32::from_rgb(0xF3, 0xEC, 0xE0),
        border: Color32::from_rgb(0xD9, 0xCD, 0xBB),
        text: Color32::from_rgb(0x3A, 0x33, 0x2A),
        text_dim: Color32::from_rgb(0x7A, 0x6F, 0x5E),
        accent: Color32::from_rgb(0xC2, 0x70, 0x3D),
        accent_soft: Color32::from_rgb(0xF6, 0xE6, 0xD6),
        selection: Color32::from_rgb(0xF0, 0xDA, 0xC4),
        toolbar: Color32::from_rgb(0xFB, 0xF7, 0xF0),
        tab_active: Color32::from_rgb(0xFF, 0xFF, 0xFF),
        tab_inactive: Color32::from_rgb(0xF0, 0xE9, 0xDD),
        success: Color32::from_rgb(0x2E, 0x9E, 0x5B),
        warn: Color32::from_rgb(0xC8, 0x8A, 0x14),
        danger: Color32::from_rgb(0xC2, 0x3B, 0x2E),
        grid_header: Color32::from_rgb(0xF0, 0xE8, 0xDA),
        grid_alt: Color32::from_rgb(0xFB, 0xF6, 0xEE),
    }
}

pub fn graphite() -> Palette {
    Palette {
        bg: Color32::from_rgb(0x0E, 0x0E, 0x10),
        panel: Color32::from_rgb(0x16, 0x16, 0x18),
        panel_alt: Color32::from_rgb(0x20, 0x20, 0x24),
        border: Color32::from_rgb(0x35, 0x35, 0x3B),
        text: Color32::from_rgb(0xED, 0xED, 0xEF),
        text_dim: Color32::from_rgb(0x9A, 0x9A, 0xA2),
        accent: Color32::from_rgb(0x8A, 0xB4, 0xF8),
        accent_soft: Color32::from_rgb(0x1E, 0x2A, 0x3A),
        selection: Color32::from_rgb(0x2A, 0x3A, 0x55),
        toolbar: Color32::from_rgb(0x12, 0x12, 0x14),
        tab_active: Color32::from_rgb(0x1C, 0x1C, 0x20),
        tab_inactive: Color32::from_rgb(0x16, 0x16, 0x18),
        success: Color32::from_rgb(0x81, 0xC9, 0x95),
        warn: Color32::from_rgb(0xFD, 0xD6, 0x63),
        danger: Color32::from_rgb(0xF2, 0x8B, 0x82),
        grid_header: Color32::from_rgb(0x1E, 0x1E, 0x22),
        grid_alt: Color32::from_rgb(0x18, 0x18, 0x1C),
    }
}

pub fn palette(key: &str) -> Palette {
    match key {
        "dark" => dark(),
        "midnight" => midnight(),
        "forest" => forest(),
        "sand" => sand(),
        "graphite" => graphite(),
        _ => light(),
    }
}

pub fn apply_theme(ctx: &egui::Context, key: &str) -> Palette {
    let p = palette(key);
    let dark_mode = is_dark(key);
    let mut visuals = if dark_mode { Visuals::dark() } else { Visuals::light() };
    visuals.override_text_color = Some(p.text);
    visuals.panel_fill = p.panel;
    visuals.window_fill = p.panel;
    visuals.extreme_bg_color = p.bg;
    visuals.faint_bg_color = p.panel_alt;
    visuals.widgets.noninteractive.bg_fill = p.panel;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, p.text);
    visuals.widgets.inactive.bg_fill = p.panel_alt;
    visuals.widgets.inactive.weak_bg_fill = p.panel_alt;
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, p.text);
    visuals.widgets.hovered.bg_fill = p.accent_soft;
    visuals.widgets.hovered.weak_bg_fill = p.accent_soft;
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, p.text);
    visuals.widgets.active.bg_fill = p.accent;
    visuals.widgets.active.weak_bg_fill = p.accent;
    // 注意：egui 中 RichText::strong() 的文字颜色取自 widgets.active.text_color()，
    // 因此这里用主题前景色而非纯白，避免“标题/加粗内容字”在浅色主题下看不见或深色主题下变暗。
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, p.text);
    visuals.selection.bg_fill = p.selection;
    visuals.selection.stroke = Stroke::new(1.0, p.accent);
    visuals.window_stroke = Stroke::new(1.0, p.border);
    visuals.window_rounding = Rounding::same(8.0);
    visuals.menu_rounding = Rounding::same(6.0);
    // 同时写入“浅色/深色”两套样式，并显式设置当前主题，确保所有面板/控件都使用本主题的
    // 文字与背景色（内容字不因主题切换而变得难读）。
    let theme = if dark_mode { egui::ThemePreference::Dark } else { egui::ThemePreference::Light };
    ctx.set_theme(theme);
    let vis2 = visuals.clone();
    ctx.all_styles_mut(move |style| {
        style.visuals = vis2.clone();
        style.spacing.item_spacing = egui::vec2(6.0, 5.0);
        style.spacing.button_padding = egui::vec2(8.0, 4.0);
        style.spacing.menu_margin = egui::Margin::same(6.0);
    });
    p
}


