use eframe::egui::Color32;
use crate::color::ColorId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConsoleProfile {
    #[default]
    KonsoleBreeze,
    VgaLinux,
    Dracula,
    SolarizedDark,
    XTermDefault,
}

impl ConsoleProfile {
    pub const ALL: [ConsoleProfile; 5] = [
        ConsoleProfile::KonsoleBreeze,
        ConsoleProfile::VgaLinux,
        ConsoleProfile::Dracula,
        ConsoleProfile::SolarizedDark,
        ConsoleProfile::XTermDefault,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            ConsoleProfile::KonsoleBreeze => "Konsole (KDE Breeze)",
            ConsoleProfile::VgaLinux => "VGA / Linux Console",
            ConsoleProfile::Dracula => "Dracula",
            ConsoleProfile::SolarizedDark => "Solarized Dark",
            ConsoleProfile::XTermDefault => "xterm / Classic ANSI",
        }
    }

    pub fn canvas_bg(&self) -> Color32 {
        let (r, g, b) = match self {
            ConsoleProfile::KonsoleBreeze => (35, 38, 41), // #232629
            ConsoleProfile::VgaLinux => (0, 0, 0),         // Pure black
            ConsoleProfile::Dracula => (40, 42, 54),       // #282a36
            ConsoleProfile::SolarizedDark => (0, 43, 54),  // #002b36
            ConsoleProfile::XTermDefault => (18, 18, 18),
        };
        Color32::from_rgb(r, g, b)
    }

    pub fn cell_bg(&self) -> Color32 {
        self.canvas_bg()
    }

    pub fn color_rgb(&self, id: ColorId) -> (u8, u8, u8) {
        match self {
            ConsoleProfile::KonsoleBreeze => match id {
                ColorId::Default => (252, 252, 252),
                ColorId::Black => (35, 38, 41),
                ColorId::Red => (237, 21, 21),
                ColorId::Green => (17, 209, 22),
                ColorId::Yellow => (246, 116, 0),
                ColorId::Blue => (29, 153, 243),
                ColorId::Magenta => (155, 89, 182),
                ColorId::Cyan => (26, 188, 156),
                ColorId::White => (252, 252, 252),
                ColorId::BrightBlack => (127, 140, 141),
                ColorId::BrightRed => (192, 57, 43),
                ColorId::BrightGreen => (46, 204, 113),
                ColorId::BrightYellow => (241, 196, 15),
                ColorId::BrightBlue => (52, 152, 219),
                ColorId::BrightMagenta => (175, 122, 197),
                ColorId::BrightCyan => (26, 188, 156),
                ColorId::BrightWhite => (255, 255, 255),
            },
            ConsoleProfile::VgaLinux => match id {
                ColorId::Default => (170, 170, 170),
                ColorId::Black => (0, 0, 0),
                ColorId::Red => (170, 0, 0),
                ColorId::Green => (0, 170, 0),
                ColorId::Yellow => (170, 85, 0),
                ColorId::Blue => (0, 0, 170),
                ColorId::Magenta => (170, 0, 170),
                ColorId::Cyan => (0, 170, 170),
                ColorId::White => (170, 170, 170),
                ColorId::BrightBlack => (85, 85, 85),
                ColorId::BrightRed => (255, 85, 85),
                ColorId::BrightGreen => (85, 255, 85),
                ColorId::BrightYellow => (255, 255, 85),
                ColorId::BrightBlue => (85, 85, 255),
                ColorId::BrightMagenta => (255, 85, 255),
                ColorId::BrightCyan => (85, 255, 255),
                ColorId::BrightWhite => (255, 255, 255),
            },
            ConsoleProfile::Dracula => match id {
                ColorId::Default => (248, 248, 242),
                ColorId::Black => (33, 34, 44),
                ColorId::Red => (255, 85, 85),
                ColorId::Green => (80, 250, 123),
                ColorId::Yellow => (241, 250, 140),
                ColorId::Blue => (189, 147, 249),
                ColorId::Magenta => (255, 121, 198),
                ColorId::Cyan => (139, 233, 253),
                ColorId::White => (248, 248, 242),
                ColorId::BrightBlack => (98, 114, 164),
                ColorId::BrightRed => (255, 110, 110),
                ColorId::BrightGreen => (105, 255, 148),
                ColorId::BrightYellow => (255, 255, 165),
                ColorId::BrightBlue => (209, 178, 255),
                ColorId::BrightMagenta => (255, 146, 214),
                ColorId::BrightCyan => (164, 255, 255),
                ColorId::BrightWhite => (255, 255, 255),
            },
            ConsoleProfile::SolarizedDark => match id {
                ColorId::Default => (131, 148, 150),
                ColorId::Black => (7, 54, 66),
                ColorId::Red => (220, 50, 47),
                ColorId::Green => (133, 153, 0),
                ColorId::Yellow => (181, 137, 0),
                ColorId::Blue => (38, 139, 210),
                ColorId::Magenta => (211, 54, 130),
                ColorId::Cyan => (42, 161, 152),
                ColorId::White => (238, 232, 213),
                ColorId::BrightBlack => (0, 43, 54),
                ColorId::BrightRed => (203, 75, 22),
                ColorId::BrightGreen => (88, 110, 117),
                ColorId::BrightYellow => (101, 123, 131),
                ColorId::BrightBlue => (131, 148, 150),
                ColorId::BrightMagenta => (108, 113, 196),
                ColorId::BrightCyan => (147, 161, 161),
                ColorId::BrightWhite => (253, 246, 227),
            },
            ConsoleProfile::XTermDefault => match id {
                ColorId::Default => (229, 229, 229),
                ColorId::Black => (20, 20, 20),
                ColorId::Red => (205, 49, 49),
                ColorId::Green => (13, 188, 121),
                ColorId::Yellow => (229, 229, 16),
                ColorId::Blue => (36, 114, 200),
                ColorId::Magenta => (188, 63, 188),
                ColorId::Cyan => (17, 168, 205),
                ColorId::White => (229, 229, 229),
                ColorId::BrightBlack => (102, 102, 102),
                ColorId::BrightRed => (241, 76, 76),
                ColorId::BrightGreen => (35, 209, 139),
                ColorId::BrightYellow => (245, 245, 67),
                ColorId::BrightBlue => (59, 142, 234),
                ColorId::BrightMagenta => (214, 112, 214),
                ColorId::BrightCyan => (41, 184, 219),
                ColorId::BrightWhite => (255, 255, 255),
            },
        }
    }
}

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
    pub scrollbar_bg: Color32,
    pub scrollbar_handle: Color32,
    pub scrollbar_handle_hover: Color32,
    pub console_profile: ConsoleProfile,
}

impl AppTheme {
    pub fn colors(&self, profile: ConsoleProfile) -> ThemeColors {
        let mut tc = match self {
            AppTheme::Light => ThemeColors {
                bg_workspace: Color32::from_rgb(235, 235, 245),
                panel_bg: Color32::from_rgb(255, 255, 255),
                panel_border: Color32::from_rgb(206, 200, 227), // #cec8e3
                canvas_bg: profile.canvas_bg(),
                cell_bg: profile.cell_bg(),
                cell_border: Color32::from_rgb(50, 48, 64),
                cell_hover: Color32::from_rgb(75, 70, 95),
                cell_selected: Color32::from_rgba_premultiplied(110, 95, 165, 90),
                text_primary: Color32::from_rgb(20, 20, 25),
                text_muted: Color32::from_rgb(110, 110, 130),
                button_bg: Color32::from_rgb(239, 239, 255), // #efefff
                button_hover: Color32::from_rgb(206, 200, 227),
                button_active: Color32::from_rgb(175, 165, 210),
                accent: Color32::from_rgb(110, 95, 165),
                shadow: Color32::from_rgba_premultiplied(110, 95, 165, 60),
                scrollbar_bg: Color32::from_rgb(222, 220, 234),
                scrollbar_handle: Color32::from_rgb(180, 172, 205),
                scrollbar_handle_hover: Color32::from_rgb(150, 138, 185),
                console_profile: profile,
            },
            AppTheme::Dark => ThemeColors {
                bg_workspace: Color32::from_rgb(24, 21, 38),
                panel_bg: Color32::from_rgb(36, 32, 56),
                panel_border: Color32::from_rgb(45, 39, 69),
                canvas_bg: profile.canvas_bg(),
                cell_bg: profile.cell_bg(),
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
                scrollbar_bg: Color32::from_rgb(18, 15, 30),
                scrollbar_handle: Color32::from_rgb(55, 48, 80),
                scrollbar_handle_hover: Color32::from_rgb(85, 75, 120),
                console_profile: profile,
            },
        };
        tc.canvas_bg = profile.canvas_bg();
        tc.cell_bg = profile.cell_bg();
        tc
    }
}
