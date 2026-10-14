use eframe::egui::{self, Stroke, Vec2};
use crate::theme::ThemeColors;
use crate::tools::ToolState;

pub struct ModalsState {
    pub show_about: bool,
    pub show_text_edit: bool,
    pub text_edit_buffer: String,
    pub show_palette_edit: bool,
    pub palette_edit_buffer: String,
    pub range_from_str: String,
    pub range_to_str: String,
    pub toast_message: Option<(String, std::time::Instant)>,
}

impl Default for ModalsState {
    fn default() -> Self {
        Self {
            show_about: false,
            show_text_edit: false,
            text_edit_buffer: String::new(),
            show_palette_edit: false,
            palette_edit_buffer: String::new(),
            range_from_str: "33".to_string(),
            range_to_str: "126".to_string(),
            toast_message: None,
        }
    }
}

pub enum ModalAction {
    ApplyText(String),
    ApplyPalette(Vec<char>),
}

pub fn render_modals(
    ctx: &egui::Context,
    state: &mut ModalsState,
    _tools: &mut ToolState,
    theme: &ThemeColors,
) -> Vec<ModalAction> {
    let mut actions = Vec::new();

    // --- About Modal ---
    if state.show_about {
        egui::Window::new("About ASCII Art Paint")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .frame(egui::Frame::new().fill(theme.panel_bg).stroke(Stroke::new(1.0, theme.panel_border)).corner_radius(16.0))
            .show(ctx, |ui| {
                ui.set_width(360.0);
                ui.vertical_centered(|ui| {
                    ui.heading("ASCII Art Paint");
                    ui.label(egui::RichText::new("Inspired by KirillLive ASCII Art Paint").color(theme.text_muted));
                    ui.separator();
                    ui.label("Full-featured ASCII and Unicode canvas editor in Rust & eframe/egui.");
                    ui.add_space(6.0);
                    ui.label(egui::RichText::new("Горячие клавиши и управление:").strong());
                    ui.label("• Shift + ЛКМ / Alt+M — выделить прямоугольную область");
                    ui.label("• Печать клавиш при выделении — ввод текста в рамку с верхнего левого угла");
                    ui.label("• Esc — снять выделение / отменить перенос блока");
                    ui.label("• ✏ Карандаш (Alt+B) / 🖌 Кисть цвета (Alt+K)");
                    ui.label("• 💧 Пипетка символа (Alt+I; -> Карандаш) / 🎨 Пипетка цвета (Alt+P; -> Кисть)");
                    ui.label("• Курсор ✥ на выделении (инструмент ⛶) — перетащить блок по холсту");
                    ui.label("• Ctrl+C / Ctrl+X — копировать / вырезать в ОЗУ и буфер");
                    ui.label("• Ctrl+V — вставить блок (с сохранением цветов)");
                    ui.label("• Ctrl+Z / Ctrl+Y — отмена и повтор действий");
                    ui.label("• Delete — очистить содержимое выделения");
                    ui.add_space(8.0);
                    ui.hyperlink_to("Original Reference", "https://kirilllive.github.io/ASCII_Art_Paint/ascii_paint.html");
                    ui.hyperlink_to("GitHub Repository", "https://github.com/Kirilllive/ASCII_Art_Paint");
                    ui.add_space(16.0);
                    if ui.button("Close").clicked() {
                        state.show_about = false;
                    }
                });
            });
    }

    // --- Text Edit Modal ---
    if state.show_text_edit {
        let screen_rect = ctx.input(|i| i.viewport_rect());
        let modal_w = (screen_rect.width() - 80.0).clamp(400.0, 1000.0);
        let modal_h = (screen_rect.height() - 80.0).clamp(300.0, 700.0);

        egui::Window::new("Text Input")
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(modal_w, modal_h))
            .frame(egui::Frame::new().fill(theme.panel_bg).stroke(Stroke::new(1.0, theme.panel_border)).corner_radius(16.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new("Paste or edit your ASCII text directly:").color(theme.text_muted));
                    ui.add_space(4.0);

                    egui::ScrollArea::both()
                        .max_height(modal_h - 110.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut state.text_edit_buffer)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(24),
                            );
                        });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Apply to Canvas").clicked() {
                            actions.push(ModalAction::ApplyText(state.text_edit_buffer.clone()));
                            state.show_text_edit = false;
                        }
                        if ui.button("Cancel").clicked() {
                            state.show_text_edit = false;
                        }
                    });
                });
            });
    }

    // --- Palette Edit Modal ---
    if state.show_palette_edit {
        egui::Window::new("Edit Symbol Palette")
            .collapsible(false)
            .resizable(true)
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .default_size(Vec2::new(500.0, 420.0))
            .frame(egui::Frame::new().fill(theme.panel_bg).stroke(Stroke::new(1.0, theme.panel_border)).corner_radius(16.0))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Palette Characters:");
                    ui.add(
                        egui::TextEdit::multiline(&mut state.palette_edit_buffer)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .desired_rows(6),
                    );

                    ui.separator();
                    ui.label(egui::RichText::new("Add Character Range (Unicode decimal or hex):").strong());

                    ui.horizontal(|ui| {
                        ui.label("From:");
                        ui.add(egui::TextEdit::singleline(&mut state.range_from_str).desired_width(70.0));
                        ui.label("To:");
                        ui.add(egui::TextEdit::singleline(&mut state.range_to_str).desired_width(70.0));

                        if ui.button("Append Range").clicked() {
                            let parse_num = |s: &str| -> Option<u32> {
                                if s.starts_with("0x") || s.starts_with("0X") {
                                    u32::from_str_radix(&s[2..], 16).ok()
                                } else {
                                    s.parse::<u32>().ok()
                                }
                            };

                            if let (Some(from), Some(to)) = (parse_num(&state.range_from_str), parse_num(&state.range_to_str)) {
                                let start = from.min(to);
                                let end = from.max(to).min(start + 500); // safety cap
                                for code in start..=end {
                                    if let Some(ch) = char::from_u32(code) {
                                        if !ch.is_control() && !state.palette_edit_buffer.contains(ch) {
                                            state.palette_edit_buffer.push(ch);
                                        }
                                    }
                                }
                            }
                        }
                    });

                    ui.separator();
                    ui.horizontal(|ui| {
                        if ui.button("Apply Palette").clicked() {
                            let chars: Vec<char> = state.palette_edit_buffer.chars().filter(|c| !c.is_control()).collect();
                            if !chars.is_empty() {
                                actions.push(ModalAction::ApplyPalette(chars));
                            }
                            state.show_palette_edit = false;
                        }
                        if ui.button("Reset to Default").clicked() {
                            state.palette_edit_buffer = crate::tools::DEFAULT_PALETTE.to_string();
                        }
                        if ui.button("Cancel").clicked() {
                            state.show_palette_edit = false;
                        }
                    });
                });
            });
    }

    // --- Toast Message Notification ---
    if let Some((msg, created_at)) = &state.toast_message {
        if created_at.elapsed().as_secs_f32() < 2.5 {
            egui::Area::new(egui::Id::new("toast_overlay"))
                .anchor(egui::Align2::CENTER_BOTTOM, Vec2::new(0.0, -40.0))
                .order(egui::Order::Tooltip)
                .show(ctx, |ui| {
                    egui::Frame::new()
                        .fill(theme.panel_bg)
                        .stroke(Stroke::new(1.0, theme.panel_border))
                        .corner_radius(20.0)
                        .inner_margin(egui::Margin::symmetric(24, 8))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(msg)
                                    .color(theme.text_primary)
                                    .strong(),
                            );
                        });
                });
        } else {
            state.toast_message = None;
        }
    }

    actions
}
