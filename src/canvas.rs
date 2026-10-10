use eframe::egui::{self, Pos2, Rect, Sense, Stroke, Vec2};
use crate::model::{CanvasGrid, Direction, RectBounds};
use crate::theme::ThemeColors;
use crate::tools::{Tool, ToolState};

pub struct ViewportState {
    pub pan_offset: Vec2,
    pub zoom: f32,
    pub cell_w: f32,
    pub cell_h: f32,
    pub hovered_cell: Option<(usize, usize)>,
    pub selection: Option<RectBounds>,
    pub selection_drag_start: Option<(usize, usize)>,
    pub is_dragging_stroke: bool,
    pub is_panning: bool,
    pub last_drag_pos: Option<Pos2>,
    pub show_context_menu: bool,
    pub context_menu_pos: Pos2,
    pub context_menu_cell: Option<(usize, usize)>,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self {
            pan_offset: Vec2::new(100.0, 70.0),
            zoom: 1.0,
            cell_w: 11.0,
            cell_h: 18.0,
            hovered_cell: None,
            selection: None,
            selection_drag_start: None,
            is_dragging_stroke: false,
            is_panning: false,
            last_drag_pos: None,
            show_context_menu: false,
            context_menu_pos: Pos2::ZERO,
            context_menu_cell: None,
        }
    }
}

pub enum CanvasAction {
    StrokeStarted,
    CellPainted(usize, usize, char),
    FloodFill(usize, usize),
    PipetteSampled(char),
    AddCells(Direction, usize),
    DelCells(Direction, usize),
    Cut,
    Copy,
    Paste,
    Delete,
    Crop,
    OpenTextEdit,
    Deselect,
}

impl ViewportState {
    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + 0.1).min(4.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom - 0.1).max(0.25);
    }

    pub fn zoom_reset(&mut self) {
        self.zoom = 1.0;
    }

    pub fn effective_cell_size(&self) -> Vec2 {
        Vec2::new(self.cell_w * self.zoom, self.cell_h * self.zoom)
    }

    pub fn screen_to_cell(&self, screen_pos: Pos2, canvas_rect: Rect, grid_w: usize, grid_h: usize) -> Option<(usize, usize)> {
        let cell_size = self.effective_cell_size();
        let rel_x = screen_pos.x - canvas_rect.min.x;
        let rel_y = screen_pos.y - canvas_rect.min.y;

        if rel_x < 0.0 || rel_y < 0.0 {
            return None;
        }

        let col = (rel_x / cell_size.x) as usize;
        let row = (rel_y / cell_size.y) as usize;

        if col < grid_w && row < grid_h {
            Some((col, row))
        } else {
            None
        }
    }

    pub fn cell_rect(&self, col: usize, row: usize, canvas_rect: Rect) -> Rect {
        let cell_size = self.effective_cell_size();
        let min = canvas_rect.min + Vec2::new(col as f32 * cell_size.x, row as f32 * cell_size.y);
        Rect::from_min_size(min, cell_size)
    }
}

pub fn render_canvas(
    ui: &mut egui::Ui,
    grid: &CanvasGrid,
    tools: &mut ToolState,
    state: &mut ViewportState,
    theme: &ThemeColors,
) -> Vec<CanvasAction> {
    let mut actions = Vec::new();

    let available_rect = ui.available_rect_before_wrap();
    let cell_size = state.effective_cell_size();
    let canvas_size = Vec2::new(grid.width as f32 * cell_size.x, grid.height as f32 * cell_size.y);

    let canvas_min = available_rect.min + state.pan_offset;
    let canvas_rect = Rect::from_min_size(canvas_min, canvas_size);

    let (response, painter) = ui.allocate_painter(available_rect.size(), Sense::click_and_drag());

    // Mouse wheel zooming
    let scroll_delta = ui.input(|i| i.smooth_scroll_delta);
    if scroll_delta.y != 0.0 && response.hovered() {
        let zoom_change = scroll_delta.y * 0.0015;
        let old_zoom = state.zoom;
        state.zoom = (state.zoom + zoom_change).clamp(0.25, 4.0);

        if let Some(pointer_pos) = ui.input(|i| i.pointer.hover_pos()) {
            let rel = pointer_pos - canvas_rect.min;
            let ratio = state.zoom / old_zoom;
            state.pan_offset += rel * (1.0 - ratio);
        }
    }

    // Panning with Middle Click or Hand Tool
    let pointer = ui.input(|i| i.pointer.clone());
    let middle_down = pointer.button_down(egui::PointerButton::Middle);
    let primary_down = pointer.primary_down();
    let secondary_down = pointer.button_down(egui::PointerButton::Secondary);
    let secondary_clicked = pointer.secondary_clicked();

    if middle_down || (tools.current_tool == Tool::Scroll && primary_down) {
        if let Some(curr) = pointer.hover_pos() {
            if let Some(last) = state.last_drag_pos {
                let delta = curr - last;
                state.pan_offset += delta;
            }
            state.last_drag_pos = Some(curr);
            state.is_panning = true;
        }
    } else {
        state.last_drag_pos = pointer.hover_pos();
        state.is_panning = false;
    }

    // Drop shadow under canvas
    let shadow_rect = canvas_rect.expand(4.0);
    painter.rect_filled(shadow_rect, 4.0, theme.shadow);

    // Canvas background
    painter.rect_filled(canvas_rect, 0.0, theme.canvas_bg);

    // Determine hovered cell
    let mouse_pos = pointer.hover_pos();
    state.hovered_cell = mouse_pos.and_then(|pos| state.screen_to_cell(pos, canvas_rect, grid.width, grid.height));

    // Handle tool interactions
    if let Some((col, row)) = state.hovered_cell {
        if secondary_down && !state.is_panning {
            // Right-click always acts as Eraser
            if !state.is_dragging_stroke {
                state.is_dragging_stroke = true;
                actions.push(CanvasAction::StrokeStarted);
            }
            actions.push(CanvasAction::CellPainted(col, row, ' '));
        } else if primary_down && !state.is_panning {
            if !state.is_dragging_stroke {
                state.is_dragging_stroke = true;
                actions.push(CanvasAction::StrokeStarted);

                match tools.current_tool {
                    Tool::Fill => {
                        actions.push(CanvasAction::FloodFill(col, row));
                    }
                    Tool::Pipette => {
                        if let Some(ch) = grid.get(col, row) {
                            actions.push(CanvasAction::PipetteSampled(ch));
                        }
                    }
                    Tool::Select => {
                        state.selection_drag_start = Some((col, row));
                        state.selection = Some(RectBounds::new(col, row, col, row));
                    }
                    Tool::Pencil => {
                        actions.push(CanvasAction::CellPainted(col, row, tools.brush_char));
                    }
                    Tool::Eraser => {
                        actions.push(CanvasAction::CellPainted(col, row, ' '));
                    }
                    Tool::Scroll => {}
                }
            } else {
                // Continued drag
                match tools.current_tool {
                    Tool::Pencil => {
                        actions.push(CanvasAction::CellPainted(col, row, tools.brush_char));
                    }
                    Tool::Eraser => {
                        actions.push(CanvasAction::CellPainted(col, row, ' '));
                    }
                    Tool::Select => {
                        if let Some((start_c, start_r)) = state.selection_drag_start {
                            state.selection = Some(RectBounds::new(start_c, start_r, col, row));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if !primary_down && !secondary_down {
        state.is_dragging_stroke = false;
        state.selection_drag_start = None;
    }

    // Context menu trigger (Shift + Right-click)
    if secondary_clicked && ui.input(|i| i.modifiers.shift) {
        if let Some(pos) = mouse_pos {
            state.show_context_menu = true;
            state.context_menu_pos = pos;
            state.context_menu_cell = state.hovered_cell;
        }
    }

    // Visible cell frustum clipping to optimize rendering
    let font_size = (12.0 * state.zoom).clamp(6.0, 48.0);
    let font_id = egui::FontId::monospace(font_size);

    for row in 0..grid.height {
        let cell_y = canvas_rect.min.y + row as f32 * cell_size.y;
        if cell_y + cell_size.y < available_rect.min.y || cell_y > available_rect.max.y {
            continue;
        }

        for col in 0..grid.width {
            let cell_x = canvas_rect.min.x + col as f32 * cell_size.x;
            if cell_x + cell_size.x < available_rect.min.x || cell_x > available_rect.max.x {
                continue;
            }

            let cell_rect = Rect::from_min_size(Pos2::new(cell_x, cell_y), cell_size);

            // Draw cell background
            let is_hovered = state.hovered_cell == Some((col, row));
            let is_selected = state.selection.map_or(false, |b| b.contains(col, row));

            let bg_color = if is_selected {
                theme.cell_selected
            } else if is_hovered {
                theme.cell_hover
            } else {
                theme.cell_bg
            };

            painter.rect_filled(cell_rect, 0.0, bg_color);
            painter.rect_stroke(cell_rect, 0.0, Stroke::new(0.5, theme.cell_border), egui::StrokeKind::Outside);

            // Draw character
            let idx = row * grid.width + col;
            let ch = grid.cells[idx];
            if ch != ' ' {
                let text = ch.to_string();
                let text_pos = cell_rect.center();
                painter.text(
                    text_pos,
                    egui::Align2::CENTER_CENTER,
                    text,
                    font_id.clone(),
                    theme.text_primary,
                );
            }
        }
    }

    // Selection border overlay
    if let Some(sel) = state.selection {
        let top_left = state.cell_rect(sel.min_x, sel.min_y, canvas_rect);
        let bottom_right = state.cell_rect(sel.max_x, sel.max_y, canvas_rect);
        let sel_rect = Rect::from_min_max(top_left.min, bottom_right.max);
        painter.rect_stroke(sel_rect, 0.0, Stroke::new(1.5, theme.accent), egui::StrokeKind::Outside);
    }

    // Draw 4-side [+] and [-] border controls matching reference
    render_side_controls(ui, &mut actions, canvas_rect, theme);

    // Draw Context Menu if active
    if state.show_context_menu {
        render_context_menu(ui, state, &mut actions, theme);
    }

    actions
}

fn render_side_controls(
    ui: &mut egui::Ui,
    actions: &mut Vec<CanvasAction>,
    canvas_rect: Rect,
    theme: &ThemeColors,
) {
    let btn_size = Vec2::new(26.0, 26.0);

    // Helper for circular button
    let mut draw_btn = |center: Pos2, text: &'static str, dir: Direction, is_add: bool| {
        let rect = Rect::from_center_size(center, btn_size);
        let resp = ui.allocate_rect(rect, Sense::click());
        let is_hover = resp.hovered();
        let bg = if is_hover { theme.button_hover } else { theme.panel_bg };
        ui.painter().rect_filled(rect, 13.0, bg);
        ui.painter().rect_stroke(rect, 13.0, Stroke::new(1.0, theme.panel_border), egui::StrokeKind::Outside);
        ui.painter().text(
            center,
            egui::Align2::CENTER_CENTER,
            text,
            egui::FontId::monospace(14.0),
            theme.text_primary,
        );
        if resp.clicked() {
            if is_add {
                actions.push(CanvasAction::AddCells(dir, 1));
            } else {
                actions.push(CanvasAction::DelCells(dir, 1));
            }
        }
    };

    // Left border buttons
    let left_center = Pos2::new(canvas_rect.min.x - 30.0, canvas_rect.center().y);
    draw_btn(left_center - Vec2::new(0.0, 16.0), "+", Direction::Left, true);
    draw_btn(left_center + Vec2::new(0.0, 16.0), "-", Direction::Left, false);

    // Right border buttons
    let right_center = Pos2::new(canvas_rect.max.x + 30.0, canvas_rect.center().y);
    draw_btn(right_center - Vec2::new(0.0, 16.0), "+", Direction::Right, true);
    draw_btn(right_center + Vec2::new(0.0, 16.0), "-", Direction::Right, false);

    // Top border buttons
    let top_center = Pos2::new(canvas_rect.center().x, canvas_rect.min.y - 30.0);
    draw_btn(top_center - Vec2::new(16.0, 0.0), "+", Direction::Top, true);
    draw_btn(top_center + Vec2::new(16.0, 0.0), "-", Direction::Top, false);

    // Bottom border buttons
    let bottom_center = Pos2::new(canvas_rect.center().x, canvas_rect.max.y + 30.0);
    draw_btn(bottom_center - Vec2::new(16.0, 0.0), "+", Direction::Bottom, true);
    draw_btn(bottom_center + Vec2::new(16.0, 0.0), "-", Direction::Bottom, false);
}

fn render_context_menu(
    ui: &mut egui::Ui,
    state: &mut ViewportState,
    actions: &mut Vec<CanvasAction>,
    theme: &ThemeColors,
) {
    let menu_rect = Rect::from_min_size(state.context_menu_pos, Vec2::new(120.0, 190.0));
    let popup_id = ui.make_persistent_id("canvas_context_menu");

    let _area_resp = egui::Area::new(popup_id)
        .fixed_pos(state.context_menu_pos)
        .order(egui::Order::Foreground)
        .show(ui.ctx(), |ui| {
            egui::Frame::new()
                .fill(theme.panel_bg)
                .stroke(Stroke::new(1.0, theme.panel_border))
                .corner_radius(8.0)
                .show(ui, |ui| {
                    ui.set_width(110.0);
                    if ui.button("✂ Cut").clicked() {
                        actions.push(CanvasAction::Cut);
                        state.show_context_menu = false;
                    }
                    if ui.button("📋 Copy").clicked() {
                        actions.push(CanvasAction::Copy);
                        state.show_context_menu = false;
                    }
                    if ui.button("📥 Paste").clicked() {
                        actions.push(CanvasAction::Paste);
                        state.show_context_menu = false;
                    }
                    if ui.button("🗑 Delete").clicked() {
                        actions.push(CanvasAction::Delete);
                        state.show_context_menu = false;
                    }
                    ui.separator();
                    if ui.button("📝 Edit text").clicked() {
                        actions.push(CanvasAction::OpenTextEdit);
                        state.show_context_menu = false;
                    }
                    if ui.button("✂ Crop").clicked() {
                        actions.push(CanvasAction::Crop);
                        state.show_context_menu = false;
                    }
                    if ui.button("✕ Deselect").clicked() {
                        actions.push(CanvasAction::Deselect);
                        state.show_context_menu = false;
                    }
                });
        });

    if ui.input(|i| i.pointer.primary_clicked()) && !menu_rect.contains(ui.input(|i| i.pointer.hover_pos().unwrap_or(Pos2::ZERO))) {
        state.show_context_menu = false;
    }
}
