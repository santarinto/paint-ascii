use eframe::egui::{self, Color32, Sense, Stroke, Vec2};
use crate::theme::ThemeColors;
use crate::tools::{Tool, ToolState};

pub fn render_left_toolbar(
    ui: &mut egui::Ui,
    tools: &mut ToolState,
    theme: &ThemeColors,
) {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(0.0, 4.0);

        let tool_list = [
            Tool::Scroll,
            Tool::Zoom,
            Tool::Pencil,
            Tool::Eraser,
            Tool::Fill,
            Tool::Pipette,
            Tool::Select,
            Tool::DeleteRow,
            Tool::DeleteCol,
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
            tools.brush_color.to_color32(theme),
        );

        resp.on_hover_text(format!("Active Brush: '{}' ({})", tools.brush_char, tools.brush_color.name()));

        // Active brush color indicator swatch
        let color_swatch_size = Vec2::new(34.0, 16.0);
        let (rect_col, resp_col) = ui.allocate_exact_size(color_swatch_size, Sense::hover());
        ui.painter().rect_filled(rect_col, 4.0, tools.brush_color.preview_color32(theme.console_profile));
        ui.painter().rect_stroke(rect_col, 4.0, Stroke::new(1.0, theme.panel_border), egui::StrokeKind::Outside);
        resp_col.on_hover_text(format!("Brush Color: {}", tools.brush_color.name()));
    });
}
