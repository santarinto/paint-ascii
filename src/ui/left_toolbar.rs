use eframe::egui::{self, Color32, Sense, Stroke, Vec2};
use crate::theme::ThemeColors;
use crate::tools::{Tool, ToolState};

pub fn render_left_toolbar(
    ui: &mut egui::Ui,
    tools: &mut ToolState,
    theme: &ThemeColors,
) {
    egui::Frame::new()
        .fill(theme.panel_bg)
        .stroke(Stroke::new(1.0, theme.panel_border))
        .corner_radius(16.0)
        .inner_margin(egui::Margin::symmetric(4, 6))
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.0, 4.0);

                let tool_list = [
                    Tool::Scroll,
                    Tool::Pencil,
                    Tool::Eraser,
                    Tool::Fill,
                    Tool::Pipette,
                    Tool::Select,
                ];

                for tool in tool_list {
                    let is_selected = tools.current_tool == tool;
                    let btn_size = Vec2::new(34.0, 34.0);
                    let (rect, resp) = ui.allocate_exact_size(btn_size, Sense::click());

                    let bg = if is_selected {
                        theme.button_active
                    } else if resp.hovered() {
                        theme.button_hover
                    } else {
                        Color32::TRANSPARENT
                    };

                    ui.painter().rect_filled(rect, 8.0, bg);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        tool.icon(),
                        egui::FontId::proportional(16.0),
                        theme.text_primary,
                    );

                    let resp = resp.on_hover_text(tool.name());
                    if resp.clicked() {
                        tools.current_tool = tool;
                    }
                }

                ui.separator();

                // Active brush indicator
                let brush_size = Vec2::new(34.0, 34.0);
                let (rect, resp) = ui.allocate_exact_size(brush_size, Sense::click());
                let bg = if resp.hovered() {
                    theme.button_hover
                } else {
                    theme.button_bg
                };

                ui.painter().rect_filled(rect, 8.0, bg);
                ui.painter().rect_stroke(rect, 8.0, Stroke::new(1.0, theme.panel_border), egui::StrokeKind::Outside);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    tools.brush_char.to_string(),
                    egui::FontId::monospace(18.0),
                    theme.text_primary,
                );

                resp.on_hover_text(format!("Active Brush: '{}' (Type any key to change)", tools.brush_char));
            });
        });
}
