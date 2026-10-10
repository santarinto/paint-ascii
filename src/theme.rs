use eframe::egui::Color32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub bg_workspace: Color32,
    pub panel_bg: Color32,
    pub panel_border: Color32,
    pub canvas_bg: Color32,
    pub cell_bg: Color32,
    pub cell_border: Color32,
    pub cell_hover: Color32,
    pub cell_selected: Color32,
    pub text_primary: Color32,
    pub text_muted: Color32,
    pub button_bg: Color32,
    pub button_hover: Color32,
    pub button_active: Color32,
    pub accent: Color32,
    pub shadow: Color32,
}

impl AppTheme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            AppTheme::Light => ThemeColors {
                bg_workspace: Color32::from_rgb(235, 235, 245),
                panel_bg: Color32::from_rgb(255, 255, 255),
                panel_border: Color32::from_rgb(206, 200, 227), // #cec8e3
                canvas_bg: Color32::from_rgb(255, 255, 255),
                cell_bg: Color32::from_rgba_premultiplied(255, 255, 255, 204),
                cell_border: Color32::from_rgb(230, 230, 240),
                cell_hover: Color32::from_rgb(206, 200, 227),
                cell_selected: Color32::from_rgba_premultiplied(110, 95, 165, 80),
                text_primary: Color32::from_rgb(20, 20, 25),
                text_muted: Color32::from_rgb(110, 110, 130),
                button_bg: Color32::from_rgb(239, 239, 255), // #efefff
                button_hover: Color32::from_rgb(206, 200, 227),
                button_active: Color32::from_rgb(175, 165, 210),
                accent: Color32::from_rgb(110, 95, 165),
                shadow: Color32::from_rgba_premultiplied(110, 95, 165, 60),
            },
            AppTheme::Dark => ThemeColors {
                bg_workspace: Color32::from_rgb(24, 21, 38),
                panel_bg: Color32::from_rgb(36, 32, 56),
                panel_border: Color32::from_rgb(45, 39, 69),
                canvas_bg: Color32::from_rgb(30, 26, 48),
                cell_bg: Color32::from_rgba_premultiplied(36, 32, 56, 204),
                cell_border: Color32::from_rgb(50, 44, 78),
                cell_hover: Color32::from_rgb(70, 60, 105),
                cell_selected: Color32::from_rgba_premultiplied(160, 140, 230, 90),
                text_primary: Color32::from_rgb(245, 245, 250),
                text_muted: Color32::from_rgb(170, 165, 190),
                button_bg: Color32::from_rgb(45, 39, 69),
                button_hover: Color32::from_rgb(65, 55, 95),
                button_active: Color32::from_rgb(90, 75, 130),
                accent: Color32::from_rgb(160, 140, 230),
                shadow: Color32::from_rgba_premultiplied(0, 0, 0, 100),
            },
        }
    }
}
