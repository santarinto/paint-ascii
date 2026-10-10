use eframe::egui::{self, Color32, Sense, Vec2};
use crate::canvas::ViewportState;
use crate::model::CanvasGrid;
use crate::theme::ThemeColors;

pub fn render_scene_controls(
    ui: &mut egui::Ui,
    viewport: &mut ViewportState,
    theme: &ThemeColors,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(4.0, 0.0);

        let mut btn = |text: &'static str, tooltip: &'static str| -> bool {
            let btn_size = Vec2::new(26.0, 22.0);
            let (rect, resp) = ui.allocate_exact_size(btn_size, Sense::click());
            let is_hover = resp.hovered();
            let bg = if is_hover {
                theme.button_hover
            } else {
                Color32::TRANSPARENT
            };

            ui.painter().rect_filled(rect, 4.0, bg);
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(13.0),
                theme.text_primary,
            );

            let resp = resp.on_hover_text(tooltip);
            resp.clicked()
        };

        if btn("-", "Zoom Out") {
            viewport.zoom_out();
        }
        if btn("◎", format!("Reset Zoom ({:.0}%)", viewport.zoom * 100.0).leak()) {
            viewport.zoom_reset();
        }
        if btn("+", "Zoom In") {
            viewport.zoom_in();
        }
        ui.label(
            egui::RichText::new(format!("{:.0}%", viewport.zoom * 100.0))
                .color(theme.text_muted)
                .size(11.0),
        );
    });
}

pub fn render_status_bar(
    ui: &mut egui::Ui,
    grid: &CanvasGrid,
    viewport: &ViewportState,
    theme: &ThemeColors,
) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = Vec2::new(14.0, 0.0);

        // Grid size
        ui.label(
            egui::RichText::new(format!("Cells: {}×{}", grid.width, grid.height))
                .color(theme.text_primary)
                .size(11.0),
        );

        // Cursor position
        if let Some((x, y)) = viewport.hovered_cell {
            ui.label(
                egui::RichText::new(format!("X: {}  Y: {}", x, y))
                    .color(theme.text_primary)
                    .size(11.0),
            );
        } else {
            ui.label(
                egui::RichText::new("X: -  Y: -")
                    .color(theme.text_muted)
                    .size(11.0),
            );
        }

        // Selection size
        if let Some(sel) = viewport.selection {
            ui.label(
                egui::RichText::new(format!("Size: {}×{}", sel.width(), sel.height()))
                    .color(theme.accent)
                    .size(11.0),
            );
        }
    });
}
