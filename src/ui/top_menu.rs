use eframe::egui::{self, Color32, Sense, Stroke, Vec2};
use crate::color::ColorId;
use crate::model::{ColShiftMode, RowShiftMode};
use crate::theme::ThemeColors;
use crate::tools::{Tool, ToolState};

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
    tools: &mut ToolState,
    theme: &ThemeColors,
    can_undo: bool,
    can_redo: bool,
    hovered_cell: Option<(usize, usize)>,
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

        // --- Contextual Settings for DeleteRow / DeleteCol ---
        if tools.current_tool == Tool::DeleteRow {
            ui.separator();
            ui.label(egui::RichText::new("Строка:").size(12.0).color(theme.text_muted));
            for mode in [RowShiftMode::ShiftUp, RowShiftMode::ShiftDown, RowShiftMode::Clear] {
                let is_sel = tools.row_shift_mode == mode;
                let bg = if is_sel { theme.button_active } else { Color32::TRANSPARENT };
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(76.0, 26.0), Sense::click());
                if resp.hovered() && !is_sel {
                    ui.painter().rect_filled(rect, 6.0, theme.button_hover);
                } else if is_sel {
                    ui.painter().rect_filled(rect, 6.0, bg);
                    ui.painter().rect_stroke(rect, 6.0, Stroke::new(1.0, theme.accent), egui::StrokeKind::Outside);
                }
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    mode.name(),
                    egui::FontId::proportional(12.0),
                    theme.text_primary,
                );
                if resp.clicked() {
                    tools.row_shift_mode = mode;
                }
            }
        } else if tools.current_tool == Tool::DeleteCol {
            ui.separator();
            ui.label(egui::RichText::new("Столбец:").size(12.0).color(theme.text_muted));
            for mode in [ColShiftMode::ShiftLeft, ColShiftMode::ShiftRight, ColShiftMode::Clear] {
                let is_sel = tools.col_shift_mode == mode;
                let bg = if is_sel { theme.button_active } else { Color32::TRANSPARENT };
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(76.0, 26.0), Sense::click());
                if resp.hovered() && !is_sel {
                    ui.painter().rect_filled(rect, 6.0, theme.button_hover);
                } else if is_sel {
                    ui.painter().rect_filled(rect, 6.0, bg);
                    ui.painter().rect_stroke(rect, 6.0, Stroke::new(1.0, theme.accent), egui::StrokeKind::Outside);
                }
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    mode.name(),
                    egui::FontId::proportional(12.0),
                    theme.text_primary,
                );
                if resp.clicked() {
                    tools.col_shift_mode = mode;
                }
            }
        }

        ui.separator();

        // --- 16-Color Fixed ANSI Palette in a Single Horizontal Row ---
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing = Vec2::new(3.0, 0.0);
            for &color in &ColorId::PALETTE_STANDARD {
                render_color_swatch(ui, tools, color, theme);
            }
            ui.add_space(4.0);
            for &color in &ColorId::PALETTE_BRIGHT {
                render_color_swatch(ui, tools, color, theme);
            }
        });

        // --- Hovered Cell Coordinates on the Right ---
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if let Some((col, row)) = hovered_cell {
                ui.label(
                    egui::RichText::new(format!("📍 X: {:>2}  Y: {:>2}", col, row))
                        .monospace()
                        .size(13.0)
                        .color(theme.text_primary),
                );
            } else {
                ui.label(
                    egui::RichText::new("📍 X: --  Y: --")
                        .monospace()
                        .size(13.0)
                        .color(theme.text_muted),
                );
            }
        });
    });

    triggered_action
}

fn render_color_swatch(
    ui: &mut egui::Ui,
    tools: &mut ToolState,
    color: ColorId,
    theme: &ThemeColors,
) {
    let size = Vec2::new(16.0, 22.0);
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click());
    let is_selected = tools.brush_color == color;

    ui.painter().rect_filled(rect, 3.0, color.preview_color32());

    let stroke = if is_selected {
        Stroke::new(2.0, Color32::WHITE)
    } else if resp.hovered() {
        Stroke::new(1.2, theme.accent)
    } else {
        Stroke::new(0.5, theme.panel_border)
    };
    ui.painter().rect_stroke(rect, 3.0, stroke, egui::StrokeKind::Outside);

    let resp = resp.on_hover_text(color.name());
    if resp.clicked() {
        tools.brush_color = color;
    }
}
