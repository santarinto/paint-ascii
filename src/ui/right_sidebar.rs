use eframe::egui::{self, Sense, Stroke, Vec2};
use crate::model::{Anchor, CanvasGrid};
use crate::theme::ThemeColors;
use crate::tools::ToolState;

pub struct RightSidebarState {
    pub selected_font_idx: usize,
    // Resize controls
    pub resize_w_str: String,
    pub resize_h_str: String,
    pub resize_proportional: bool,
    pub resize_aspect_ratio: f32,
    pub resize_anchor: Anchor,
    // Replace controls
    pub replace_from_str: String,
    pub replace_to_str: String,
    // Image to ASCII controls
    pub loaded_image_rgba: Option<image::RgbaImage>,
    pub loaded_image_texture: Option<egui::TextureHandle>,
    pub img_brightness: f32,
    pub img_contrast: f32,
    pub img_palette_str: String,
    // Export PNG
    pub png_w_str: String,
    pub png_h_str: String,
    pub png_proportional: bool,
}

impl Default for RightSidebarState {
    fn default() -> Self {
        Self {
            selected_font_idx: 0,
            resize_w_str: "60".to_string(),
            resize_h_str: "30".to_string(),
            resize_proportional: true,
            resize_aspect_ratio: 2.0,
            resize_anchor: Anchor::TopLeft,
            replace_from_str: "".to_string(),
            replace_to_str: "".to_string(),
            loaded_image_rgba: None,
            loaded_image_texture: None,
            img_brightness: 0.0,
            img_contrast: 0.0,
            img_palette_str: " .:-=+*#%@".to_string(),
            png_w_str: "800".to_string(),
            png_h_str: "600".to_string(),
            png_proportional: true,
        }
    }
}

pub enum SidebarAction {
    OpenPaletteEditor,
    ResizeCanvas(usize, usize, Anchor),
    ReplaceChar(char, char),
    LoadImage,
    DeleteImage,
    ConvertImgToAscii,
    ConvertImgToBraille,
    SavePng(usize, usize),
}

pub fn render_right_sidebar(
    ui: &mut egui::Ui,
    _grid: &CanvasGrid,
    tools: &mut ToolState,
    state: &mut RightSidebarState,
    theme: &ThemeColors,
) -> Vec<SidebarAction> {
    let mut actions = Vec::new();

    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.set_width(240.0);
        ui.spacing_mut().item_spacing = Vec2::new(0.0, 8.0);

        // --- Font Selector ---
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Font:").color(theme.text_primary));
            let fonts = ["Monospace", "Courier New", "Lucida Console", "Consolas"];
            egui::ComboBox::from_id_salt("font_selector")
                .selected_text(fonts[state.selected_font_idx])
                .show_ui(ui, |ui| {
                    for (i, &f) in fonts.iter().enumerate() {
                        ui.selectable_value(&mut state.selected_font_idx, i, f);
                    }
                });
        });

        ui.separator();

        // --- Symbol Palette Grid ---
        ui.label(egui::RichText::new("Symbol Palette").strong().color(theme.text_primary));
        
        let cols_count = 7;
        let cell_btn_size = Vec2::new(30.0, 30.0);

        egui::Grid::new("palette_grid")
            .spacing(Vec2::new(3.0, 3.0))
            .show(ui, |ui| {
                for (idx, &ch) in tools.palette.iter().enumerate() {
                    let is_active = tools.brush_char == ch;
                    let (rect, resp) = ui.allocate_exact_size(cell_btn_size, Sense::click());
                    let bg = if is_active {
                        theme.button_active
                    } else if resp.hovered() {
                        theme.button_hover
                    } else {
                        theme.button_bg
                    };

                    ui.painter().rect_filled(rect, 4.0, bg);
                    ui.painter().rect_stroke(rect, 4.0, Stroke::new(0.5, theme.panel_border), egui::StrokeKind::Outside);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        ch.to_string(),
                        egui::FontId::monospace(14.0),
                        theme.text_primary,
                    );

                    if resp.clicked() {
                        tools.brush_char = ch;
                    }

                    if (idx + 1) % cols_count == 0 {
                        ui.end_row();
                    }
                }
            });

        let edit_pal_btn = egui::Button::new("Edit Symbol Palette")
            .fill(theme.button_bg)
            .corner_radius(8.0);
        if ui.add_sized([240.0, 28.0], edit_pal_btn).clicked() {
            actions.push(SidebarAction::OpenPaletteEditor);
        }

        ui.separator();

        // --- Image to ASCII Section ---
        ui.label(egui::RichText::new("Image to ASCII / Braille").strong().color(theme.text_primary));

        if state.loaded_image_rgba.is_none() {
            let load_btn = egui::Button::new("📂 Load Image...")
                .fill(theme.button_bg)
                .corner_radius(8.0);
            if ui.add_sized([240.0, 28.0], load_btn).clicked() {
                actions.push(SidebarAction::LoadImage);
            }
        } else {
            // Show preview texture if available
            if let Some(tex) = &state.loaded_image_texture {
                let aspect = tex.size()[1] as f32 / tex.size()[0] as f32;
                let preview_w = 220.0;
                let preview_h = (preview_w * aspect).min(140.0);
                ui.image((tex.id(), Vec2::new(preview_w, preview_h)));
            }

            ui.add(egui::Slider::new(&mut state.img_brightness, -100.0..=100.0).text("Brightness"));
            ui.add(egui::Slider::new(&mut state.img_contrast, -100.0..=100.0).text("Contrast"));

            ui.horizontal(|ui| {
                ui.label("Palette:");
                ui.text_edit_singleline(&mut state.img_palette_str);
            });

            ui.horizontal(|ui| {
                if ui.button("ASCII Art").clicked() {
                    actions.push(SidebarAction::ConvertImgToAscii);
                }
                if ui.button("Braille Art").clicked() {
                    actions.push(SidebarAction::ConvertImgToBraille);
                }
                if ui.button("🗑 Del").clicked() {
                    actions.push(SidebarAction::DeleteImage);
                }
            });
        }

        ui.separator();

        // --- Resize Canvas Section ---
        ui.label(egui::RichText::new("Resize Canvas").strong().color(theme.text_primary));

        ui.horizontal(|ui| {
            ui.label("Anchor:");
            let anchors = [
                (Anchor::TopLeft, "↖"),
                (Anchor::TopRight, "↗"),
                (Anchor::Center, "•"),
                (Anchor::BottomLeft, "↙"),
                (Anchor::BottomRight, "↘"),
            ];
            for (anc, label) in anchors {
                let is_sel = state.resize_anchor == anc;
                if ui.selectable_label(is_sel, label).clicked() {
                    state.resize_anchor = anc;
                }
            }
        });

        ui.horizontal(|ui| {
            ui.label("W:");
            let w_edit = ui.add(egui::TextEdit::singleline(&mut state.resize_w_str).desired_width(50.0));
            if w_edit.changed() && state.resize_proportional {
                if let Ok(w_val) = state.resize_w_str.parse::<f32>() {
                    let new_h = (w_val / state.resize_aspect_ratio).round() as usize;
                    state.resize_h_str = new_h.to_string();
                }
            }

            ui.label("H:");
            let h_edit = ui.add(egui::TextEdit::singleline(&mut state.resize_h_str).desired_width(50.0));
            if h_edit.changed() && state.resize_proportional {
                if let Ok(h_val) = state.resize_h_str.parse::<f32>() {
                    let new_w = (h_val * state.resize_aspect_ratio).round() as usize;
                    state.resize_w_str = new_w.to_string();
                }
            }

            ui.checkbox(&mut state.resize_proportional, "🔗");
        });

        let resize_btn = egui::Button::new("Resize Canvas")
            .fill(theme.button_bg)
            .corner_radius(8.0);
        if ui.add_sized([240.0, 28.0], resize_btn).clicked() {
            if let (Ok(w), Ok(h)) = (state.resize_w_str.parse::<usize>(), state.resize_h_str.parse::<usize>()) {
                actions.push(SidebarAction::ResizeCanvas(w, h, state.resize_anchor));
            }
        }

        ui.separator();

        // --- Find & Replace Section ---
        ui.label(egui::RichText::new("Replace Characters").strong().color(theme.text_primary));

        ui.horizontal(|ui| {
            ui.label("What:");
            ui.add(egui::TextEdit::singleline(&mut state.replace_from_str).desired_width(36.0));

            if ui.button("⇄").clicked() {
                std::mem::swap(&mut state.replace_from_str, &mut state.replace_to_str);
            }

            ui.label("With:");
            ui.add(egui::TextEdit::singleline(&mut state.replace_to_str).desired_width(36.0));
        });

        let replace_btn = egui::Button::new("Replace")
            .fill(theme.button_bg)
            .corner_radius(8.0);
        if ui.add_sized([240.0, 28.0], replace_btn).clicked() {
            if let (Some(from_ch), Some(to_ch)) = (state.replace_from_str.chars().next(), state.replace_to_str.chars().next()) {
                actions.push(SidebarAction::ReplaceChar(from_ch, to_ch));
            }
        }

        ui.separator();

        // --- Save PNG Section ---
        ui.label(egui::RichText::new("Save Image PNG").strong().color(theme.text_primary));

        ui.horizontal(|ui| {
            ui.label("W:");
            ui.add(egui::TextEdit::singleline(&mut state.png_w_str).desired_width(50.0));
            ui.label("H:");
            ui.add(egui::TextEdit::singleline(&mut state.png_h_str).desired_width(50.0));
            ui.checkbox(&mut state.png_proportional, "🔗");
        });

        let save_png_btn = egui::Button::new("Save Image PNG")
            .fill(theme.button_bg)
            .corner_radius(8.0);
        if ui.add_sized([240.0, 28.0], save_png_btn).clicked() {
            let w = state.png_w_str.parse::<usize>().unwrap_or(800);
            let h = state.png_h_str.parse::<usize>().unwrap_or(600);
            actions.push(SidebarAction::SavePng(w, h));
        }
    });

    actions
}
