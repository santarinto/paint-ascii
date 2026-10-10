mod canvas;
mod image_converter;
mod model;
mod theme;
mod tools;
mod ui;

use eframe::egui::{self, Margin, Pos2};
use canvas::{render_canvas, CanvasAction, ViewportState};
use image_converter::{convert_image_to_ascii, convert_image_to_braille, render_grid_to_png};
use model::{CanvasGrid, HistoryManager};
use theme::AppTheme;
use tools::{Tool, ToolState};
use ui::bottom_controls::{render_scene_controls, render_status_bar};
use ui::left_toolbar::render_left_toolbar;
use ui::modals::{render_modals, ModalAction, ModalsState};
use ui::right_sidebar::{render_right_sidebar, RightSidebarState, SidebarAction};
use ui::top_menu::{render_top_menu, MenuAction};

struct AsciiPaintApp {
    grid: CanvasGrid,
    history: HistoryManager,
    tools: ToolState,
    viewport: ViewportState,
    theme: AppTheme,
    sidebar: RightSidebarState,
    modals: ModalsState,
}

impl AsciiPaintApp {
    fn new() -> Self {
        Self {
            grid: CanvasGrid::new(20, 30),
            history: HistoryManager::new(50),
            tools: ToolState::default(),
            viewport: ViewportState::default(),
            theme: AppTheme::Light,
            sidebar: RightSidebarState::default(),
            modals: ModalsState::default(),
        }
    }

    fn copy_to_clipboard(&mut self, ctx: &egui::Context) {
        let text = if let Some(sel) = self.viewport.selection {
            self.grid.copy_region(sel)
        } else {
            self.grid.to_ascii_string()
        };
        ctx.copy_text(text);
        self.set_toast("Copied to clipboard");
    }

    fn get_clipboard_text(&self, ctx: &egui::Context) -> Option<String> {
        let from_event = ctx.input(|i| {
            i.events.iter().find_map(|e| {
                if let egui::Event::Paste(s) = e {
                    Some(s.clone())
                } else {
                    None
                }
            })
        });

        from_event.or_else(|| {
            let mut clipboard = arboard::Clipboard::new().ok()?;
            clipboard.get_text().ok()
        })
    }

    fn paste_from_clipboard(&mut self, ctx: &egui::Context) {
        if let Some(text) = self.get_clipboard_text(ctx) {
            if text.trim().is_empty() {
                self.set_toast("Clipboard is empty");
                return;
            }
            self.history.push_state(self.grid.clone());
            let (dest_x, dest_y) = self.viewport.selection
                .map(|s| (s.min_x, s.min_y))
                .or(self.viewport.hovered_cell)
                .unwrap_or((0, 0));

            self.grid.paste_region_expanding(dest_x, dest_y, &text, false);
            self.sidebar.resize_w_str = self.grid.width.to_string();
            self.sidebar.resize_h_str = self.grid.height.to_string();
            self.set_toast(format!("Pasted & expanded canvas ({}×{})", self.grid.width, self.grid.height));
        } else {
            self.set_toast("Clipboard is empty");
        }
    }

    fn import_from_clipboard(&mut self, ctx: &egui::Context) {
        if let Some(text) = self.get_clipboard_text(ctx) {
            if text.trim().is_empty() {
                self.set_toast("Clipboard is empty");
                return;
            }
            self.history.push_state(self.grid.clone());
            self.grid.paste_region_expanding(0, 0, &text, false);
            self.sidebar.resize_w_str = self.grid.width.to_string();
            self.sidebar.resize_h_str = self.grid.height.to_string();
            self.set_toast(format!("Imported from clipboard ({}×{})", self.grid.width, self.grid.height));
        } else {
            self.set_toast("Clipboard is empty");
        }
    }

    fn set_toast(&mut self, msg: impl Into<String>) {
        self.modals.toast_message = Some((msg.into(), std::time::Instant::now()));
    }

    fn handle_keyboard_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());

        // Do not intercept hotkeys if modal or text field has focus
        if self.modals.show_text_edit || self.modals.show_palette_edit {
            return;
        }

        let alt = input.modifiers.alt;
        let ctrl = input.modifiers.command || input.modifiers.ctrl;
        let shift = input.modifiers.shift;

        // Tools shortcuts
        if alt {
            if input.key_pressed(egui::Key::H) {
                self.tools.current_tool = Tool::Scroll;
            } else if input.key_pressed(egui::Key::B) {
                self.tools.current_tool = Tool::Pencil;
            } else if input.key_pressed(egui::Key::E) {
                self.tools.current_tool = Tool::Eraser;
            } else if input.key_pressed(egui::Key::G) {
                self.tools.current_tool = Tool::Fill;
            } else if input.key_pressed(egui::Key::I) {
                self.tools.current_tool = Tool::Pipette;
            } else if input.key_pressed(egui::Key::M) {
                self.tools.current_tool = Tool::Select;
            } else if input.key_pressed(egui::Key::C) {
                self.copy_to_clipboard(ctx);
            } else if input.key_pressed(egui::Key::V) {
                self.paste_from_clipboard(ctx);
            } else if input.key_pressed(egui::Key::S) {
                self.save_txt_file();
            } else if input.key_pressed(egui::Key::O) {
                self.load_txt_file();
            } else if input.key_pressed(egui::Key::Z) {
                if shift {
                    self.redo();
                } else {
                    self.undo();
                }
            }
        } else if ctrl {
            if input.key_pressed(egui::Key::Z) {
                if shift {
                    self.redo();
                } else {
                    self.undo();
                }
            } else if input.key_pressed(egui::Key::Y) {
                self.redo();
            } else if input.key_pressed(egui::Key::C) {
                self.copy_to_clipboard(ctx);
            } else if input.key_pressed(egui::Key::V) {
                if shift {
                    self.import_from_clipboard(ctx);
                } else {
                    self.paste_from_clipboard(ctx);
                }
            } else if input.key_pressed(egui::Key::S) {
                self.save_txt_file();
            } else if input.key_pressed(egui::Key::O) {
                self.load_txt_file();
            }
        } else {
            if input.key_pressed(egui::Key::Escape) {
                self.viewport.selection = None;
            } else if input.key_pressed(egui::Key::Delete) || input.key_pressed(egui::Key::Backspace) {
                if let Some(sel) = self.viewport.selection {
                    self.history.push_state(self.grid.clone());
                    self.grid.clear_region(sel);
                }
            }

            // Any printable key changes active brush character!
            for event in &input.events {
                if let egui::Event::Text(text) = event {
                    if let Some(ch) = text.chars().next() {
                        if !ch.is_control() {
                            self.tools.brush_char = ch;
                        }
                    }
                }
            }
        }
    }

    fn undo(&mut self) {
        if let Some(prev) = self.history.undo(self.grid.clone()) {
            self.grid = prev;
            self.set_toast("Undo");
        }
    }

    fn redo(&mut self) {
        if let Some(next) = self.history.redo(self.grid.clone()) {
            self.grid = next;
            self.set_toast("Redo");
        }
    }

    fn load_txt_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text file", &["txt", "asc", "ascii"])
            .pick_file()
        {
            if let Ok(content) = std::fs::read_to_string(&path) {
                self.history.push_state(self.grid.clone());
                self.grid = CanvasGrid::from_ascii_str(&content);
                self.sidebar.resize_w_str = self.grid.width.to_string();
                self.sidebar.resize_h_str = self.grid.height.to_string();
                self.set_toast(format!("Loaded: {}", path.file_name().unwrap_or_default().to_string_lossy()));
            }
        }
    }

    fn save_txt_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Text file", &["txt"])
            .save_file()
        {
            let content = self.grid.to_ascii_string();
            if std::fs::write(&path, content).is_ok() {
                self.set_toast("File saved successfully");
            }
        }
    }
}

impl eframe::App for AsciiPaintApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let theme_colors = self.theme.colors();
        let ctx = ui.ctx().clone();

        self.handle_keyboard_shortcuts(&ctx);

        // --- Right Sidebar Panel ---
        let mut sidebar_actions = Vec::new();
        egui::Panel::right("right_panel")
            .frame(
                egui::Frame::new()
                    .fill(theme_colors.panel_bg)
                    .stroke(egui::Stroke::new(1.0, theme_colors.panel_border))
                    .inner_margin(Margin::same(12)),
            )
            .resizable(false)
            .exact_size(390.0)
            .show(ui, |ui| {
                sidebar_actions = render_right_sidebar(
                    ui,
                    &self.grid,
                    &mut self.tools,
                    &mut self.sidebar,
                    &theme_colors,
                );
            });

        // --- Central Panel (Workspace and Canvas) ---
        let mut canvas_actions = Vec::new();
        let mut menu_action = None;

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme_colors.bg_workspace))
            .show(ui, |ui| {
                // Render Canvas
                canvas_actions = render_canvas(
                    ui,
                    &self.grid,
                    &mut self.tools,
                    &mut self.viewport,
                    &theme_colors,
                );

                // Top Floating Menu
                egui::Area::new(egui::Id::new("top_menu_area"))
                    .fixed_pos(Pos2::new(16.0, 16.0))
                    .order(egui::Order::Foreground)
                    .show(&ctx, |ui| {
                        menu_action = render_top_menu(
                            ui,
                            &theme_colors,
                            self.history.can_undo(),
                            self.history.can_redo(),
                        );
                    });

                // Left Floating Toolbar
                egui::Area::new(egui::Id::new("left_toolbar_area"))
                    .fixed_pos(Pos2::new(16.0, 80.0))
                    .order(egui::Order::Foreground)
                    .show(&ctx, |ui| {
                        render_left_toolbar(ui, &mut self.tools, &theme_colors);
                    });

                // Bottom Floating Scene Controls
                let screen = ctx.input(|i| i.viewport_rect());
                egui::Area::new(egui::Id::new("bottom_scene_controls"))
                    .fixed_pos(Pos2::new(screen.max.x - 420.0, screen.max.y - 56.0))
                    .order(egui::Order::Foreground)
                    .show(&ctx, |ui| {
                        render_scene_controls(ui, &mut self.viewport, &theme_colors);
                    });

                // Bottom Status Bar
                egui::Area::new(egui::Id::new("bottom_status_bar"))
                    .fixed_pos(Pos2::new(16.0, screen.max.y - 48.0))
                    .order(egui::Order::Foreground)
                    .show(&ctx, |ui| {
                        render_status_bar(ui, &self.grid, &self.viewport, &theme_colors);
                    });
            });

        // --- Process Menu Actions ---
        if let Some(action) = menu_action {
            match action {
                MenuAction::New => {
                    self.history.push_state(self.grid.clone());
                    self.grid = CanvasGrid::new(20, 30);
                    self.sidebar.resize_w_str = "20".to_string();
                    self.sidebar.resize_h_str = "30".to_string();
                    self.set_toast("Created new 20×30 canvas");
                }
                MenuAction::Load => {
                    self.load_txt_file();
                }
                MenuAction::Save => {
                    self.save_txt_file();
                }
                MenuAction::Undo => {
                    self.undo();
                }
                MenuAction::Redo => {
                    self.redo();
                }
                MenuAction::About => {
                    self.modals.show_about = true;
                }
                MenuAction::TextInput => {
                    self.modals.show_text_edit = true;
                    self.modals.text_edit_buffer = self.grid.to_ascii_string();
                }
                MenuAction::Copy => {
                    self.copy_to_clipboard(&ctx);
                }
                MenuAction::ImportClipboard => {
                    self.import_from_clipboard(&ctx);
                }
                MenuAction::ToggleTheme => {
                    self.theme = match self.theme {
                        AppTheme::Light => AppTheme::Dark,
                        AppTheme::Dark => AppTheme::Light,
                    };
                }
            }
        }

        // --- Process Canvas Actions ---
        for action in canvas_actions {
            match action {
                CanvasAction::StrokeStarted => {
                    self.history.push_state(self.grid.clone());
                }
                CanvasAction::CellPainted(x, y, ch) => {
                    self.grid.set(x, y, ch);
                }
                CanvasAction::FloodFill(x, y) => {
                    self.history.push_state(self.grid.clone());
                    self.grid.flood_fill(x, y, self.tools.brush_char, self.viewport.selection);
                }
                CanvasAction::PipetteSampled(ch) => {
                    self.tools.brush_char = ch;
                    self.set_toast(format!("Picked brush: '{}'", ch));
                }
                CanvasAction::AddCells(dir, count) => {
                    self.history.push_state(self.grid.clone());
                    self.grid.add_cells(dir, count);
                    self.sidebar.resize_w_str = self.grid.width.to_string();
                    self.sidebar.resize_h_str = self.grid.height.to_string();
                }
                CanvasAction::DelCells(dir, count) => {
                    self.history.push_state(self.grid.clone());
                    self.grid.del_cells(dir, count);
                    self.sidebar.resize_w_str = self.grid.width.to_string();
                    self.sidebar.resize_h_str = self.grid.height.to_string();
                }
                CanvasAction::Cut => {
                    self.copy_to_clipboard(&ctx);
                    if let Some(sel) = self.viewport.selection {
                        self.history.push_state(self.grid.clone());
                        self.grid.clear_region(sel);
                    }
                }
                CanvasAction::Copy => {
                    self.copy_to_clipboard(&ctx);
                }
                CanvasAction::Paste => {
                    self.paste_from_clipboard(&ctx);
                }
                CanvasAction::Delete => {
                    if let Some(sel) = self.viewport.selection {
                        self.history.push_state(self.grid.clone());
                        self.grid.clear_region(sel);
                    }
                }
                CanvasAction::Crop => {
                    if let Some(sel) = self.viewport.selection {
                        self.history.push_state(self.grid.clone());
                        self.grid.crop(sel);
                        self.viewport.selection = None;
                        self.sidebar.resize_w_str = self.grid.width.to_string();
                        self.sidebar.resize_h_str = self.grid.height.to_string();
                        self.set_toast("Cropped canvas to selection");
                    }
                }
                CanvasAction::OpenTextEdit => {
                    self.modals.show_text_edit = true;
                    self.modals.text_edit_buffer = self.grid.to_ascii_string();
                }
                CanvasAction::Deselect => {
                    self.viewport.selection = None;
                }
            }
        }

        // --- Process Sidebar Actions ---
        for action in sidebar_actions {
            match action {
                SidebarAction::OpenPaletteEditor => {
                    self.modals.show_palette_edit = true;
                    self.modals.palette_edit_buffer = self.tools.palette.iter().collect();
                }
                SidebarAction::ResizeCanvas(w, h, anchor) => {
                    self.history.push_state(self.grid.clone());
                    self.grid.resize(w, h, anchor);
                    self.set_toast(format!("Resized canvas to {}×{}", w, h));
                }
                SidebarAction::ReplaceChar(from, to) => {
                    self.history.push_state(self.grid.clone());
                    self.grid.replace_char(from, to, self.viewport.selection);
                    self.set_toast(format!("Replaced '{}' with '{}'", from, to));
                }
                SidebarAction::LoadImage => {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Image", &["png", "jpg", "jpeg", "webp"])
                        .pick_file()
                    {
                        if let Ok(dyn_img) = image::open(&path) {
                            let rgba = dyn_img.to_rgba8();
                            let (w, h) = (rgba.width() as usize, rgba.height() as usize);
                            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                                [w, h],
                                rgba.as_raw(),
                            );
                            let tex = ctx.load_texture("user_img", color_image, Default::default());
                            self.sidebar.loaded_image_rgba = Some(rgba);
                            self.sidebar.loaded_image_texture = Some(tex);
                            self.set_toast("Loaded image");
                        }
                    }
                }
                SidebarAction::DeleteImage => {
                    self.sidebar.loaded_image_rgba = None;
                    self.sidebar.loaded_image_texture = None;
                    self.set_toast("Deleted loaded image");
                }
                SidebarAction::ConvertImgToAscii => {
                    if let Some(img) = &self.sidebar.loaded_image_rgba {
                        let pal_chars: Vec<char> = self.sidebar.img_palette_str.chars().collect();
                        self.history.push_state(self.grid.clone());
                        self.grid = convert_image_to_ascii(
                            img,
                            self.grid.width,
                            self.grid.height,
                            self.sidebar.img_brightness,
                            self.sidebar.img_contrast,
                            &pal_chars,
                        );
                        self.set_toast("Converted image to ASCII Art");
                    }
                }
                SidebarAction::ConvertImgToBraille => {
                    if let Some(img) = &self.sidebar.loaded_image_rgba {
                        self.history.push_state(self.grid.clone());
                        self.grid = convert_image_to_braille(
                            img,
                            self.grid.width,
                            self.grid.height,
                            self.sidebar.img_brightness,
                            self.sidebar.img_contrast,
                        );
                        self.set_toast("Converted image to Braille Art");
                    }
                }
                SidebarAction::SavePng(w, h) => {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("PNG image", &["png"])
                        .save_file()
                    {
                        let png = render_grid_to_png(&self.grid, w as u32, h as u32);
                        if png.save(&path).is_ok() {
                            self.set_toast("Saved PNG image");
                        }
                    }
                }
                SidebarAction::PasteFavoritesFromClipboard => {
                    if let Some(text) = self.get_clipboard_text(&ctx) {
                        let mut added = 0;
                        for ch in text.chars() {
                            if !ch.is_control() && !self.sidebar.favorites.contains(&ch) {
                                self.sidebar.favorites.push(ch);
                                added += 1;
                            }
                        }
                        self.set_toast(format!("Added {} symbols to favorites", added));
                    }
                }
            }
        }

        // --- Render Modals ---
        let modal_actions = render_modals(&ctx, &mut self.modals, &mut self.tools, &theme_colors);
        for action in modal_actions {
            match action {
                ModalAction::ApplyText(text) => {
                    self.history.push_state(self.grid.clone());
                    self.grid = CanvasGrid::from_ascii_str(&text);
                    self.sidebar.resize_w_str = self.grid.width.to_string();
                    self.sidebar.resize_h_str = self.grid.height.to_string();
                    self.set_toast("Applied text to canvas");
                }
                ModalAction::ApplyPalette(chars) => {
                    self.tools.palette = chars;
                    self.set_toast("Updated symbol palette");
                }
            }
        }
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("ASCII Art Paint")
            .with_inner_size([1200.0, 760.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ASCII Art Paint",
        native_options,
        Box::new(|_cc| Ok(Box::new(AsciiPaintApp::new()))),
    )
}
