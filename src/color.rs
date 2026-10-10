use eframe::egui::Color32;
use crate::theme::ThemeColors;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorId {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl ColorId {
    pub const PALETTE_STANDARD: [ColorId; 8] = [
        ColorId::Black,
        ColorId::Red,
        ColorId::Green,
        ColorId::Yellow,
        ColorId::Blue,
        ColorId::Magenta,
        ColorId::Cyan,
        ColorId::White,
    ];

    pub const PALETTE_BRIGHT: [ColorId; 8] = [
        ColorId::BrightBlack,
        ColorId::BrightRed,
        ColorId::BrightGreen,
        ColorId::BrightYellow,
        ColorId::BrightBlue,
        ColorId::BrightMagenta,
        ColorId::BrightCyan,
        ColorId::BrightWhite,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ColorId::Default => "Default",
            ColorId::Black => "black",
            ColorId::Red => "red",
            ColorId::Green => "green",
            ColorId::Yellow => "yellow",
            ColorId::Blue => "blue",
            ColorId::Magenta => "magenta",
            ColorId::Cyan => "cyan",
            ColorId::White => "white",
            ColorId::BrightBlack => "bright black",
            ColorId::BrightRed => "bright red",
            ColorId::BrightGreen => "bright green",
            ColorId::BrightYellow => "bright yellow",
            ColorId::BrightBlue => "bright blue",
            ColorId::BrightMagenta => "bright magenta",
            ColorId::BrightCyan => "bright cyan",
            ColorId::BrightWhite => "bright white",
        }
    }

    pub fn to_color32(&self, theme: &ThemeColors) -> Color32 {
        match self {
            ColorId::Default => theme.text_primary,
            ColorId::Black => Color32::from_rgb(0, 0, 0),
            ColorId::Red => Color32::from_rgb(205, 49, 49),
            ColorId::Green => Color32::from_rgb(13, 188, 121),
            ColorId::Yellow => Color32::from_rgb(229, 229, 16),
            ColorId::Blue => Color32::from_rgb(36, 114, 200),
            ColorId::Magenta => Color32::from_rgb(188, 63, 188),
            ColorId::Cyan => Color32::from_rgb(17, 168, 205),
            ColorId::White => Color32::from_rgb(229, 229, 229),
            ColorId::BrightBlack => Color32::from_rgb(102, 102, 102),
            ColorId::BrightRed => Color32::from_rgb(241, 76, 76),
            ColorId::BrightGreen => Color32::from_rgb(35, 209, 139),
            ColorId::BrightYellow => Color32::from_rgb(245, 245, 67),
            ColorId::BrightBlue => Color32::from_rgb(59, 142, 234),
            ColorId::BrightMagenta => Color32::from_rgb(214, 112, 214),
            ColorId::BrightCyan => Color32::from_rgb(41, 184, 219),
            ColorId::BrightWhite => Color32::from_rgb(255, 255, 255),
        }
    }

    pub fn preview_color32(&self) -> Color32 {
        match self {
            ColorId::Default => Color32::from_rgb(180, 180, 180),
            ColorId::Black => Color32::from_rgb(20, 20, 20),
            ColorId::Red => Color32::from_rgb(205, 49, 49),
            ColorId::Green => Color32::from_rgb(13, 188, 121),
            ColorId::Yellow => Color32::from_rgb(229, 229, 16),
            ColorId::Blue => Color32::from_rgb(36, 114, 200),
            ColorId::Magenta => Color32::from_rgb(188, 63, 188),
            ColorId::Cyan => Color32::from_rgb(17, 168, 205),
            ColorId::White => Color32::from_rgb(229, 229, 229),
            ColorId::BrightBlack => Color32::from_rgb(102, 102, 102),
            ColorId::BrightRed => Color32::from_rgb(241, 76, 76),
            ColorId::BrightGreen => Color32::from_rgb(35, 209, 139),
            ColorId::BrightYellow => Color32::from_rgb(245, 245, 67),
            ColorId::BrightBlue => Color32::from_rgb(59, 142, 234),
            ColorId::BrightMagenta => Color32::from_rgb(214, 112, 214),
            ColorId::BrightCyan => Color32::from_rgb(41, 184, 219),
            ColorId::BrightWhite => Color32::from_rgb(255, 255, 255),
        }
    }
}
