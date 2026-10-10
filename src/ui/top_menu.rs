use eframe::egui::{self, Color32, Sense, Vec2};
use crate::theme::ThemeColors;

pub enum MenuAction {
    New,
    Load,
    Save,
    Undo,
    Redo,
    About,
    TextInput,
    Copy,
    ImportClipboard,
    ToggleTheme,
}

pub fn render_top_menu(
    ui: &mut egui::Ui,
    theme: &ThemeColors,
    can_undo: bool,
    can_redo: bool,
) -> Option<MenuAction> {
    let mut triggered_action = None;

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

        let mut btn = |icon: &'static str, tooltip: &'static str, enabled: bool| -> bool {
                    let btn_size = Vec2::new(32.0, 32.0);
                    let (rect, resp) = ui.allocate_exact_size(btn_size, Sense::click());
                    let is_hover = resp.hovered() && enabled;
                    let bg = if is_hover {
                        theme.button_hover
                    } else {
                        Color32::TRANSPARENT
                    };

                    let text_color = if enabled {
                        theme.text_primary
                    } else {
                        theme.text_muted
                    };

                    ui.painter().rect_filled(rect, 8.0, bg);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        icon,
                        egui::FontId::proportional(16.0),
                        text_color,
                    );

                    let resp = resp.on_hover_text(tooltip);
                    enabled && resp.clicked()
                };

                if btn("📄", "New txt file", true) {
                    triggered_action = Some(MenuAction::New);
                }
                if btn("📂", "Load txt file (Alt+O)", true) {
                    triggered_action = Some(MenuAction::Load);
                }
                if btn("💾", "Save txt file (Alt+S)", true) {
                    triggered_action = Some(MenuAction::Save);
                }
                if btn("↩", "Undo (Alt+Z)", can_undo) {
                    triggered_action = Some(MenuAction::Undo);
                }
                if btn("↪", "Redo (Alt+Shift+Z)", can_redo) {
                    triggered_action = Some(MenuAction::Redo);
                }
                if btn("ℹ", "About ASCII Art paint", true) {
                    triggered_action = Some(MenuAction::About);
                }
                if btn("✏", "Text input modal", true) {
                    triggered_action = Some(MenuAction::TextInput);
                }
                if btn("📋", "Copy text to clipboard (Alt+C)", true) {
                    triggered_action = Some(MenuAction::Copy);
                }
                if btn("📥", "Import from clipboard (auto-expands canvas) (Alt+V)", true) {
                    triggered_action = Some(MenuAction::ImportClipboard);
                }
                if btn("🌓", "Toggle light / dark theme", true) {
                    triggered_action = Some(MenuAction::ToggleTheme);
                }
            });

    triggered_action
}
