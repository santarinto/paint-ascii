use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};
use crate::color::ColorId;
use crate::model::{CanvasGrid, ColShiftMode, Direction, RectBounds, RowShiftMode};
use crate::theme::ThemeColors;
use crate::tools::{Tool, ToolState};

#[derive(Debug, Clone)]
pub struct MovingBlockState {
    pub bounds: RectBounds,
    pub grab_offset: (usize, usize),
    pub current_top_left: (usize, usize),
    pub cells: Vec<char>,
    pub colors: Vec<ColorId>,
    pub grid_before_pickup: CanvasGrid,
}

pub struct ViewportState {
    #[allow(dead_code)]
    pub pan_offset: Vec2,
    pub zoom: f32,
    pub cell_w: f32,
    pub cell_h: f32,
    pub hovered_cell: Option<(usize, usize)>,
    pub selection: Option<RectBounds>,
    pub selection_drag_start: Option<(usize, usize)>,
    pub text_cursor: Option<(usize, usize)>,
    pub is_typing: bool,
    pub moving_block: Option<MovingBlockState>,
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
            text_cursor: None,
            is_typing: false,
            moving_block: None,
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
    CellPainted(usize, usize, char, ColorId),
    CellColorPainted(usize, usize, ColorId),
    FloodFill(usize, usize, char, ColorId),
    PipetteSampled(char, ColorId),
    ColorPipetteSampled(ColorId),
    DeleteRow(usize, RowShiftMode),
    DeleteCol(usize, ColShiftMode),
    AddCells(Direction, usize),
    DelCells(Direction, usize),
    Cut,
    Copy,
    Paste,
    Delete,
    Crop,
    OpenTextEdit,
    Deselect,
    BlockMoved {
        grid_before: CanvasGrid,
        new_grid: CanvasGrid,
        new_bounds: RectBounds,
    },
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

    let cell_size = state.effective_cell_size();
    let canvas_size = Vec2::new(grid.width as f32 * cell_size.x, grid.height as f32 * cell_size.y);
    let margin = 50.0;

    let ctrl_down = ui.input(|i| i.modifiers.ctrl || i.modifiers.command);

    egui::ScrollArea::both()
        .auto_shrink([false, false])
        .scroll([!ctrl_down, !ctrl_down])
        .show(ui, |ui| {
            let viewport_size = ui.available_size();
            let content_w = (canvas_size.x + margin * 2.0).max(viewport_size.x);
            let content_h = (canvas_size.y + margin * 2.0).max(viewport_size.y);
            let total_content_size = Vec2::new(content_w, content_h);

            let (response, painter) = ui.allocate_painter(total_content_size, Sense::click_and_drag());

            let offset_x = margin.max((viewport_size.x - canvas_size.x) / 2.0);
            let offset_y = margin.max((viewport_size.y - canvas_size.y) / 2.0);
            let canvas_min = response.rect.min + Vec2::new(offset_x, offset_y);
            let canvas_rect = Rect::from_min_size(canvas_min, canvas_size);

            // Ctrl + Mouse wheel zooming
            let zoom_factor = ui.input(|i| {
                let z = i.zoom_delta();
                if (z - 1.0).abs() > 0.001 {
                    return z;
                }
                if ctrl_down {
                    for ev in &i.events {
                        if let egui::Event::MouseWheel { delta, .. } = ev {
                            if delta.y > 0.0 {
                                return 1.15;
                            } else if delta.y < 0.0 {
                                return 1.0 / 1.15;
                            }
                        }
                    }
                    if i.smooth_scroll_delta.y > 0.0 {
                        return 1.15;
                    } else if i.smooth_scroll_delta.y < 0.0 {
                        return 1.0 / 1.15;
                    }
                }
                1.0
            });

            if (zoom_factor - 1.0).abs() > 0.001 {
                if zoom_factor > 1.0 {
                    state.zoom = (state.zoom * 1.15).min(5.0);
                } else {
                    state.zoom = (state.zoom / 1.15).max(0.25);
                }
            }

            // Panning with Space + Left Click or Hand Tool
            let space_down = ui.input(|i| i.key_down(egui::Key::Space));
            let pointer = ui.input(|i| i.pointer.clone());
            let middle_down = pointer.button_down(egui::PointerButton::Middle);
            let middle_clicked = pointer.button_clicked(egui::PointerButton::Middle);
            let primary_down = pointer.primary_down();
            let secondary_down = pointer.button_down(egui::PointerButton::Secondary);
            let secondary_clicked = pointer.secondary_clicked();

            let is_pan_active = (tools.current_tool == Tool::Scroll && primary_down) || (space_down && primary_down);
            if is_pan_active {
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
                if let Some(curr) = pointer.hover_pos() {
                    if let Some(last) = state.last_drag_pos {
                        let delta = curr - last;
                        ui.scroll_with_delta(delta);
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

            let shift_down = ui.input(|i| i.modifiers.shift);

            // Handle block movement (drag & drop of selected region)
            let is_on_selection = state.hovered_cell.map_or(false, |(c, r)| {
                state.selection.map_or(false, |sel| sel.contains(c, r))
            });

            if state.moving_block.is_some() && primary_down {
                // Currently dragging a block over the canvas
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Move);
                if let Some(pos) = mouse_pos {
                    let rel_x = pos.x - canvas_rect.min.x;
                    let rel_y = pos.y - canvas_rect.min.y;
                    let col = (rel_x / cell_size.x).floor() as isize;
                    let row = (rel_y / cell_size.y).floor() as isize;

                    if let Some(ref mut moving) = state.moving_block {
                        let block_w = moving.bounds.width();
                        let block_h = moving.bounds.height();
                        let max_x = grid.width.saturating_sub(block_w) as isize;
                        let max_y = grid.height.saturating_sub(block_h) as isize;

                        let target_x = (col - moving.grab_offset.0 as isize).clamp(0, max_x.max(0)) as usize;
                        let target_y = (row - moving.grab_offset.1 as isize).clamp(0, max_y.max(0)) as usize;

                        moving.current_top_left = (target_x, target_y);
                        state.selection = Some(RectBounds::new(
                            target_x,
                            target_y,
                            target_x + block_w - 1,
                            target_y + block_h - 1,
                        ));
                    }
                }
            } else if !primary_down && state.moving_block.is_some() {
                // Mouse released while moving a block -> commit move!
                if let Some(moving) = state.moving_block.take() {
                    let (new_min_x, new_min_y) = moving.current_top_left;
                    let (orig_min_x, orig_min_y) = (moving.bounds.min_x, moving.bounds.min_y);
                    let block_w = moving.bounds.width();
                    let block_h = moving.bounds.height();
                    let new_bounds = RectBounds::new(
                        new_min_x,
                        new_min_y,
                        new_min_x + block_w - 1,
                        new_min_y + block_h - 1,
                    );

                    if new_min_x != orig_min_x || new_min_y != orig_min_y {
                        let mut new_grid = moving.grid_before_pickup.clone();
                        // Clear original bounds
                        for r in moving.bounds.min_y..=moving.bounds.max_y {
                            for c in moving.bounds.min_x..=moving.bounds.max_x {
                                new_grid.set_colored(c, r, ' ', ColorId::Default);
                            }
                        }
                        // Write block at new destination
                        for r in 0..block_h {
                            for c in 0..block_w {
                                let idx = r * block_w + c;
                                new_grid.set_colored(new_min_x + c, new_min_y + r, moving.cells[idx], moving.colors[idx]);
                            }
                        }

                        actions.push(CanvasAction::BlockMoved {
                            grid_before: moving.grid_before_pickup,
                            new_grid,
                            new_bounds,
                        });
                    }
                    state.selection = Some(new_bounds);
                }
            } else if is_on_selection && !shift_down && !state.is_panning && tools.current_tool == Tool::Select {
                // Hovering over selected block with Select tool: show 4-arrow move cursor and initiate drag on click
                ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Move);
                if primary_down && !state.is_dragging_stroke {
                    if let Some((col, row)) = state.hovered_cell {
                        state.text_cursor = Some((col, row));
                        if let Some(sel) = state.selection {
                            let grab_offset = (col - sel.min_x, row - sel.min_y);
                            let block_w = sel.width();
                            let block_h = sel.height();
                            let mut block_cells = Vec::with_capacity(block_w * block_h);
                            let mut block_colors = Vec::with_capacity(block_w * block_h);
                            for r in sel.min_y..=sel.max_y {
                                for c in sel.min_x..=sel.max_x {
                                    let idx = r * grid.width + c;
                                    block_cells.push(grid.cells[idx]);
                                    block_colors.push(grid.colors[idx]);
                                }
                            }
                            state.moving_block = Some(MovingBlockState {
                                bounds: sel,
                                grab_offset,
                                current_top_left: (sel.min_x, sel.min_y),
                                cells: block_cells,
                                colors: block_colors,
                                grid_before_pickup: grid.clone(),
                            });
                            state.is_dragging_stroke = true;
                        }
                    }
                }
            } else if let Some((col, row)) = state.hovered_cell {
                // Cursor icon overrides for tools
                if tools.current_tool == Tool::ColorBrush
                    || tools.current_tool == Tool::ColorPipette
                    || tools.current_tool == Tool::Pipette
                {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Crosshair);
                }

                // Regular tool interactions
                if (shift_down || state.selection_drag_start.is_some()) && !state.is_panning && tools.current_tool != Tool::Scroll {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Crosshair);
                    if primary_down {
                        if !state.is_dragging_stroke {
                            state.is_dragging_stroke = true;
                            state.selection_drag_start = Some((col, row));
                            state.selection = Some(RectBounds::new(col, row, col, row));
                            state.text_cursor = Some((col, row));
                            state.is_typing = false;
                        } else if let Some((start_c, start_r)) = state.selection_drag_start {
                            let bounds = RectBounds::new(start_c, start_r, col, row);
                            state.selection = Some(bounds);
                            state.text_cursor = Some((bounds.min_x, bounds.min_y));
                        }
                    }
                } else if tools.current_tool == Tool::Zoom {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::ZoomIn);
                    if pointer.primary_clicked() {
                        state.zoom = (state.zoom * 1.5).min(5.0);
                    } else if pointer.secondary_clicked() && !shift_down {
                        state.zoom = (state.zoom / 1.5).max(0.25);
                    }
                } else if tools.current_tool == Tool::DeleteRow {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    if pointer.primary_clicked() {
                        actions.push(CanvasAction::DeleteRow(row, tools.row_shift_mode));
                    }
                } else if tools.current_tool == Tool::DeleteCol {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
                    if pointer.primary_clicked() {
                        actions.push(CanvasAction::DeleteCol(col, tools.col_shift_mode));
                    }
                } else if (middle_clicked || middle_down) && !state.is_panning {
                    // Middle-click always acts as Pipette (samples char AND color)
                    if let Some(ch) = grid.get(col, row) {
                        let color = grid.get_color(col, row).unwrap_or(ColorId::Default);
                        actions.push(CanvasAction::PipetteSampled(ch, color));
                    }
                } else if secondary_down && !state.is_panning {
                    // Right-click always acts as Eraser
                    if !state.is_dragging_stroke {
                        state.is_dragging_stroke = true;
                        actions.push(CanvasAction::StrokeStarted);
                    }
                    actions.push(CanvasAction::CellPainted(col, row, ' ', ColorId::Default));
                } else if primary_down && !state.is_panning {
                    if !state.is_dragging_stroke {
                        state.is_dragging_stroke = true;
                        actions.push(CanvasAction::StrokeStarted);

                        match tools.current_tool {
                            Tool::Fill => {
                                actions.push(CanvasAction::FloodFill(col, row, tools.brush_char, tools.brush_color));
                            }
                            Tool::Pipette => {
                                if let Some(ch) = grid.get(col, row) {
                                    let color = grid.get_color(col, row).unwrap_or(ColorId::Default);
                                    actions.push(CanvasAction::PipetteSampled(ch, color));
                                }
                            }
                            Tool::ColorPipette => {
                                let color = grid.get_color(col, row).unwrap_or(ColorId::Default);
                                actions.push(CanvasAction::ColorPipetteSampled(color));
                            }
                            Tool::Select => {
                                state.selection_drag_start = Some((col, row));
                                state.selection = Some(RectBounds::new(col, row, col, row));
                                state.text_cursor = Some((col, row));
                                state.is_typing = false;
                            }
                            Tool::Pencil => {
                                actions.push(CanvasAction::CellPainted(col, row, tools.brush_char, tools.brush_color));
                            }
                            Tool::ColorBrush => {
                                actions.push(CanvasAction::CellColorPainted(col, row, tools.brush_color));
                            }
                            Tool::Eraser => {
                                actions.push(CanvasAction::CellPainted(col, row, ' ', ColorId::Default));
                            }
                            Tool::Scroll | Tool::Zoom | Tool::DeleteRow | Tool::DeleteCol => {}
                        }
                    } else {
                        // Continued drag
                        match tools.current_tool {
                            Tool::Pencil => {
                                actions.push(CanvasAction::CellPainted(col, row, tools.brush_char, tools.brush_color));
                            }
                            Tool::ColorBrush => {
                                actions.push(CanvasAction::CellColorPainted(col, row, tools.brush_color));
                            }
                            Tool::Eraser => {
                                actions.push(CanvasAction::CellPainted(col, row, ' ', ColorId::Default));
                            }
                            Tool::Select => {
                                if let Some((start_c, start_r)) = state.selection_drag_start {
                                    let bounds = RectBounds::new(start_c, start_r, col, row);
                                    state.selection = Some(bounds);
                                    state.text_cursor = Some((bounds.min_x, bounds.min_y));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            if !primary_down && !secondary_down && !middle_down {
                state.is_dragging_stroke = false;
                state.selection_drag_start = None;
            }

            // Context menu trigger (Right-click on selection OR Shift + Right-click)
            if secondary_clicked {
                let on_selection = state.hovered_cell.map_or(false, |(c, r)| {
                    state.selection.map_or(false, |sel| sel.contains(c, r))
                });
                if shift_down || on_selection {
                    if let Some(pos) = mouse_pos {
                        state.show_context_menu = true;
                        state.context_menu_pos = pos;
                        state.context_menu_cell = state.hovered_cell;
                    }
                }
            }

            // Visible cell frustum clipping to optimize rendering
            let font_size = (12.0 * state.zoom).clamp(6.0, 48.0);
            let font_id = egui::FontId::monospace(font_size);
            let clip = ui.clip_rect();

            for row in 0..grid.height {
                let cell_y = canvas_rect.min.y + row as f32 * cell_size.y;
                if cell_y + cell_size.y < clip.min.y || cell_y > clip.max.y {
                    continue;
                }

                for col in 0..grid.width {
                    let cell_x = canvas_rect.min.x + col as f32 * cell_size.x;
                    if cell_x + cell_size.x < clip.min.x || cell_x > clip.max.x {
                        continue;
                    }

                    let cell_rect = Rect::from_min_size(Pos2::new(cell_x, cell_y), cell_size);

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

                    let (ch, col_id) = if let Some(ref moving) = state.moving_block {
                        let block_w = moving.bounds.width();
                        let block_h = moving.bounds.height();
                        let (top_x, top_y) = moving.current_top_left;
                        if col >= top_x && col < top_x + block_w && row >= top_y && row < top_y + block_h {
                            let block_c = col - top_x;
                            let block_r = row - top_y;
                            let idx = block_r * block_w + block_c;
                            (moving.cells[idx], moving.colors[idx])
                        } else if moving.bounds.contains(col, row) {
                            (' ', ColorId::Default)
                        } else {
                            let idx = row * grid.width + col;
                            (grid.cells[idx], grid.colors[idx])
                        }
                    } else {
                        let idx = row * grid.width + col;
                        (grid.cells[idx], grid.colors[idx])
                    };

                    if ch != ' ' {
                        let text = ch.to_string();
                        let text_pos = cell_rect.center();
                        painter.text(
                            text_pos,
                            egui::Align2::CENTER_CENTER,
                            text,
                            font_id.clone(),
                            col_id.to_color32(theme),
                        );
                    }
                }
            }

            // Row or Column hover highlight indicator when DeleteRow or DeleteCol is active
            if let Some((col, row)) = state.hovered_cell {
                if tools.current_tool == Tool::DeleteRow {
                    let row_y = canvas_rect.min.y + row as f32 * cell_size.y;
                    let row_rect = Rect::from_min_size(Pos2::new(canvas_rect.min.x, row_y), Vec2::new(canvas_size.x, cell_size.y));
                    painter.rect_filled(row_rect, 0.0, Color32::from_rgba_unmultiplied(230, 80, 80, 45));
                    painter.rect_stroke(row_rect, 0.0, Stroke::new(1.0, Color32::from_rgb(230, 80, 80)), egui::StrokeKind::Outside);
                } else if tools.current_tool == Tool::DeleteCol {
                    let col_x = canvas_rect.min.x + col as f32 * cell_size.x;
                    let col_rect = Rect::from_min_size(Pos2::new(col_x, canvas_rect.min.y), Vec2::new(cell_size.x, canvas_size.y));
                    painter.rect_filled(col_rect, 0.0, Color32::from_rgba_unmultiplied(230, 80, 80, 45));
                    painter.rect_stroke(col_rect, 0.0, Stroke::new(1.0, Color32::from_rgb(230, 80, 80)), egui::StrokeKind::Outside);
                }
            }

            // Selection border overlay and blinking text cursor
            if let Some(sel) = state.selection {
                let top_left = state.cell_rect(sel.min_x, sel.min_y, canvas_rect);
                let bottom_right = state.cell_rect(sel.max_x, sel.max_y, canvas_rect);
                let sel_rect = Rect::from_min_max(top_left.min, bottom_right.max);
                painter.rect_stroke(sel_rect, 0.0, Stroke::new(1.5, theme.accent), egui::StrokeKind::Outside);

                // Blinking text cursor inside selection
                let (cx, cy) = state.text_cursor.unwrap_or((sel.min_x, sel.min_y));
                if sel.contains(cx, cy) && state.moving_block.is_none() {
                    let cur_cell_rect = state.cell_rect(cx, cy, canvas_rect);
                    let time = ui.input(|i| i.time);
                    let is_blink_on = (time * 2.0).fract() < 0.55;
                    if is_blink_on {
                        painter.line_segment(
                            [cur_cell_rect.left_top(), cur_cell_rect.left_bottom()],
                            Stroke::new(2.5, theme.accent),
                        );
                        painter.line_segment(
                            [
                                cur_cell_rect.left_bottom() - Vec2::new(0.0, 1.0),
                                cur_cell_rect.right_bottom() - Vec2::new(0.0, 1.0),
                            ],
                            Stroke::new(2.0, theme.accent),
                        );
                    }
                    ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
                }
            }

            // Draw 4-side [+] and [-] border controls matching reference
            render_side_controls(ui, &mut actions, canvas_rect, theme);

            // Draw Context Menu if active
            if state.show_context_menu {
                render_context_menu(ui, state, &mut actions, theme);
            }
        });

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
        let id = ui.make_persistent_id(format!("side_btn_{}_{:?}", text, dir));
        let resp = ui.interact(rect, id, Sense::click());
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
