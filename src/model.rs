use crate::color::ColorId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RowShiftMode {
    #[default]
    ShiftUp,
    ShiftDown,
    Clear,
}

impl RowShiftMode {
    pub fn name(&self) -> &'static str {
        match self {
            RowShiftMode::ShiftUp => "⬆ Вверх",
            RowShiftMode::ShiftDown => "⬇ Вниз",
            RowShiftMode::Clear => "🧹 Очистить",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColShiftMode {
    #[default]
    ShiftLeft,
    ShiftRight,
    Clear,
}

impl ColShiftMode {
    pub fn name(&self) -> &'static str {
        match self {
            ColShiftMode::ShiftLeft => "⬅ Влево",
            ColShiftMode::ShiftRight => "➡ Вправо",
            ColShiftMode::Clear => "🧹 Очистить",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Anchor {
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RectBounds {
    pub min_x: usize,
    pub min_y: usize,
    pub max_x: usize,
    pub max_y: usize,
}

impl RectBounds {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Self {
        Self {
            min_x: x1.min(x2),
            min_y: y1.min(y2),
            max_x: x1.max(x2),
            max_y: y1.max(y2),
        }
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    pub fn width(&self) -> usize {
        self.max_x.saturating_sub(self.min_x) + 1
    }

    pub fn height(&self) -> usize {
        self.max_y.saturating_sub(self.min_y) + 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanvasGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<char>,
    pub colors: Vec<ColorId>,
}

impl CanvasGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        Self {
            width,
            height,
            cells: vec![' '; width * height],
            colors: vec![ColorId::Default; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = y * self.width + x;
        self.cells.get(index).copied()
    }

    pub fn get_color(&self, x: usize, y: usize) -> Option<ColorId> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = y * self.width + x;
        self.colors.get(index).copied()
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) -> bool {
        self.set_colored(x, y, c, ColorId::Default)
    }

    pub fn set_colored(&mut self, x: usize, y: usize, c: char, color: ColorId) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let index = y * self.width + x;
        self.cells[index] = c;
        self.colors[index] = color;
        true
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cells.fill(' ');
        self.colors.fill(ColorId::Default);
    }

    pub fn delete_row(&mut self, row: usize, mode: RowShiftMode) {
        if row >= self.height {
            return;
        }
        match mode {
            RowShiftMode::Clear => {
                for x in 0..self.width {
                    let idx = row * self.width + x;
                    self.cells[idx] = ' ';
                    self.colors[idx] = ColorId::Default;
                }
            }
            RowShiftMode::ShiftUp => {
                for y in row..(self.height - 1) {
                    for x in 0..self.width {
                        let dst = y * self.width + x;
                        let src = (y + 1) * self.width + x;
                        self.cells[dst] = self.cells[src];
                        self.colors[dst] = self.colors[src];
                    }
                }
                let last_y = self.height - 1;
                for x in 0..self.width {
                    let idx = last_y * self.width + x;
                    self.cells[idx] = ' ';
                    self.colors[idx] = ColorId::Default;
                }
            }
            RowShiftMode::ShiftDown => {
                for y in (1..=row).rev() {
                    for x in 0..self.width {
                        let dst = y * self.width + x;
                        let src = (y - 1) * self.width + x;
                        self.cells[dst] = self.cells[src];
                        self.colors[dst] = self.colors[src];
                    }
                }
                for x in 0..self.width {
                    self.cells[x] = ' ';
                    self.colors[x] = ColorId::Default;
                }
            }
        }
    }

    pub fn delete_col(&mut self, col: usize, mode: ColShiftMode) {
        if col >= self.width {
            return;
        }
        match mode {
            ColShiftMode::Clear => {
                for y in 0..self.height {
                    let idx = y * self.width + col;
                    self.cells[idx] = ' ';
                    self.colors[idx] = ColorId::Default;
                }
            }
            ColShiftMode::ShiftLeft => {
                for y in 0..self.height {
                    for x in col..(self.width - 1) {
                        let dst = y * self.width + x;
                        let src = y * self.width + (x + 1);
                        self.cells[dst] = self.cells[src];
                        self.colors[dst] = self.colors[src];
                    }
                    let last_x = self.width - 1;
                    let idx = y * self.width + last_x;
                    self.cells[idx] = ' ';
                    self.colors[idx] = ColorId::Default;
                }
            }
            ColShiftMode::ShiftRight => {
                for y in 0..self.height {
                    for x in (1..=col).rev() {
                        let dst = y * self.width + x;
                        let src = y * self.width + (x - 1);
                        self.cells[dst] = self.cells[src];
                        self.colors[dst] = self.colors[src];
                    }
                    let idx = y * self.width;
                    self.cells[idx] = ' ';
                    self.colors[idx] = ColorId::Default;
                }
            }
        }
    }

    pub fn to_ascii_string(&self) -> String {
        let mut result = String::with_capacity(self.height * (self.width + 1));
        for y in 0..self.height {
            for x in 0..self.width {
                result.push(self.cells[y * self.width + x]);
            }
            if y + 1 < self.height {
                result.push('\n');
            }
        }
        result
    }

    pub fn from_ascii_str(s: &str) -> Self {
        let rows = crate::ansi::parse_ansi_text(s);
        if rows.is_empty() {
            return Self::new(60, 30);
        }
        let height = rows.len();
        let width = rows.iter().map(|r| r.len()).max().unwrap_or(1).max(1);

        let mut grid = Self::new(width, height);
        for (y, row) in rows.iter().enumerate() {
            for (x, (ch, color)) in row.iter().enumerate() {
                grid.set_colored(x, y, *ch, *color);
            }
        }
        grid
    }

    pub fn add_cells(&mut self, dir: Direction, count: usize) {
        if count == 0 {
            return;
        }
        match dir {
            Direction::Left => {
                let new_width = self.width + count;
                let mut new_cells = vec![' '; new_width * self.height];
                let mut new_colors = vec![ColorId::Default; new_width * self.height];
                for y in 0..self.height {
                    for x in 0..self.width {
                        new_cells[y * new_width + (x + count)] = self.cells[y * self.width + x];
                        new_colors[y * new_width + (x + count)] = self.colors[y * self.width + x];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
                self.colors = new_colors;
            }
            Direction::Right => {
                let new_width = self.width + count;
                let mut new_cells = vec![' '; new_width * self.height];
                let mut new_colors = vec![ColorId::Default; new_width * self.height];
                for y in 0..self.height {
                    for x in 0..self.width {
                        new_cells[y * new_width + x] = self.cells[y * self.width + x];
                        new_colors[y * new_width + x] = self.colors[y * self.width + x];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
                self.colors = new_colors;
            }
            Direction::Top => {
                let new_height = self.height + count;
                let mut new_cells = vec![' '; self.width * new_height];
                let mut new_colors = vec![ColorId::Default; self.width * new_height];
                let offset = count * self.width;
                new_cells[offset..offset + self.cells.len()].copy_from_slice(&self.cells);
                new_colors[offset..offset + self.colors.len()].copy_from_slice(&self.colors);
                self.height = new_height;
                self.cells = new_cells;
                self.colors = new_colors;
            }
            Direction::Bottom => {
                let new_height = self.height + count;
                self.cells.resize(self.width * new_height, ' ');
                self.colors.resize(self.width * new_height, ColorId::Default);
                self.height = new_height;
            }
        }
    }

    pub fn del_cells(&mut self, dir: Direction, count: usize) {
        if count == 0 {
            return;
        }
        match dir {
            Direction::Left => {
                if count >= self.width {
                    return;
                }
                let new_width = self.width - count;
                let mut new_cells = vec![' '; new_width * self.height];
                let mut new_colors = vec![ColorId::Default; new_width * self.height];
                for y in 0..self.height {
                    for x in 0..new_width {
                        new_cells[y * new_width + x] = self.cells[y * self.width + (x + count)];
                        new_colors[y * new_width + x] = self.colors[y * self.width + (x + count)];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
                self.colors = new_colors;
            }
            Direction::Right => {
                if count >= self.width {
                    return;
                }
                let new_width = self.width - count;
                let mut new_cells = vec![' '; new_width * self.height];
                let mut new_colors = vec![ColorId::Default; new_width * self.height];
                for y in 0..self.height {
                    for x in 0..new_width {
                        new_cells[y * new_width + x] = self.cells[y * self.width + x];
                        new_colors[y * new_width + x] = self.colors[y * self.width + x];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
                self.colors = new_colors;
            }
            Direction::Top => {
                if count >= self.height {
                    return;
                }
                let new_height = self.height - count;
                let start = count * self.width;
                let end = self.cells.len();
                self.cells = self.cells[start..end].to_vec();
                self.colors = self.colors[start..end].to_vec();
                self.height = new_height;
            }
            Direction::Bottom => {
                if count >= self.height {
                    return;
                }
                let new_height = self.height - count;
                self.cells.truncate(self.width * new_height);
                self.colors.truncate(self.width * new_height);
                self.height = new_height;
            }
        }
    }

    pub fn crop(&mut self, bounds: RectBounds) {
        let min_x = bounds.min_x.min(self.width.saturating_sub(1));
        let max_x = bounds.max_x.min(self.width.saturating_sub(1));
        let min_y = bounds.min_y.min(self.height.saturating_sub(1));
        let max_y = bounds.max_y.min(self.height.saturating_sub(1));

        let new_w = (max_x - min_x + 1).max(1);
        let new_h = (max_y - min_y + 1).max(1);

        let mut new_cells = vec![' '; new_w * new_h];
        let mut new_colors = vec![ColorId::Default; new_w * new_h];
        for y in 0..new_h {
            for x in 0..new_w {
                let old_idx = (min_y + y) * self.width + (min_x + x);
                new_cells[y * new_w + x] = self.cells[old_idx];
                new_colors[y * new_w + x] = self.colors[old_idx];
            }
        }
        self.width = new_w;
        self.height = new_h;
        self.cells = new_cells;
        self.colors = new_colors;
    }

    pub fn resize(&mut self, new_w: usize, new_h: usize, anchor: Anchor) {
        let new_w = new_w.max(1);
        let new_h = new_h.max(1);
        if new_w == self.width && new_h == self.height {
            return;
        }

        let mut new_grid = Self::new(new_w, new_h);

        let (src_x_offset, dst_x_offset) = match anchor {
            Anchor::TopLeft | Anchor::BottomLeft => (0, 0),
            Anchor::TopRight | Anchor::BottomRight => {
                if new_w >= self.width {
                    (0, new_w - self.width)
                } else {
                    (self.width - new_w, 0)
                }
            }
            Anchor::Center => {
                if new_w >= self.width {
                    (0, (new_w - self.width) / 2)
                } else {
                    ((self.width - new_w) / 2, 0)
                }
            }
        };

        let (src_y_offset, dst_y_offset) = match anchor {
            Anchor::TopLeft | Anchor::TopRight => (0, 0),
            Anchor::BottomLeft | Anchor::BottomRight => {
                if new_h >= self.height {
                    (0, new_h - self.height)
                } else {
                    (self.height - new_h, 0)
                }
            }
            Anchor::Center => {
                if new_h >= self.height {
                    (0, (new_h - self.height) / 2)
                } else {
                    ((self.height - new_h) / 2, 0)
                }
            }
        };

        let copy_w = self.width.saturating_sub(src_x_offset).min(new_w.saturating_sub(dst_x_offset));
        let copy_h = self.height.saturating_sub(src_y_offset).min(new_h.saturating_sub(dst_y_offset));

        for y in 0..copy_h {
            for x in 0..copy_w {
                let src_idx = (src_y_offset + y) * self.width + (src_x_offset + x);
                let dst_idx = (dst_y_offset + y) * new_grid.width + (dst_x_offset + x);
                new_grid.cells[dst_idx] = self.cells[src_idx];
                new_grid.colors[dst_idx] = self.colors[src_idx];
            }
        }

        *self = new_grid;
    }

    pub fn replace_char(&mut self, from: char, to: char, bounds: Option<RectBounds>) {
        if from == to {
            return;
        }
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(b) = bounds {
                    if !b.contains(x, y) {
                        continue;
                    }
                }
                let idx = y * self.width + x;
                if self.cells[idx] == from {
                    self.cells[idx] = to;
                }
            }
        }
    }

    pub fn flood_fill(&mut self, start_x: usize, start_y: usize, fill_char: char, fill_color: ColorId, bounds: Option<RectBounds>) {
        if start_x >= self.width || start_y >= self.height {
            return;
        }
        if let Some(b) = bounds {
            if !b.contains(start_x, start_y) {
                return;
            }
        }
        let target_char = self.cells[start_y * self.width + start_x];
        let target_color = self.colors[start_y * self.width + start_x];
        if target_char == fill_char && target_color == fill_color {
            return;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start_x, start_y));
        self.cells[start_y * self.width + start_x] = fill_char;
        self.colors[start_y * self.width + start_x] = fill_color;

        let (min_x, max_x, min_y, max_y) = if let Some(b) = bounds {
            (
                b.min_x.min(self.width - 1),
                b.max_x.min(self.width - 1),
                b.min_y.min(self.height - 1),
                b.max_y.min(self.height - 1),
            )
        } else {
            (0, self.width - 1, 0, self.height - 1)
        };

        while let Some((cx, cy)) = queue.pop_front() {
            let neighbors = [
                (cx.wrapping_sub(1), cy, cx > min_x),
                (cx + 1, cy, cx < max_x),
                (cx, cy.wrapping_sub(1), cy > min_y),
                (cx, cy + 1, cy < max_y),
            ];

            for (nx, ny, valid) in neighbors {
                if valid && nx < self.width && ny < self.height {
                    let idx = ny * self.width + nx;
                    if self.cells[idx] == target_char && self.colors[idx] == target_color {
                        self.cells[idx] = fill_char;
                        self.colors[idx] = fill_color;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }
    }

    pub fn copy_region(&self, bounds: RectBounds) -> String {
        let min_x = bounds.min_x.min(self.width.saturating_sub(1));
        let max_x = bounds.max_x.min(self.width.saturating_sub(1));
        let min_y = bounds.min_y.min(self.height.saturating_sub(1));
        let max_y = bounds.max_y.min(self.height.saturating_sub(1));

        let mut lines = Vec::new();
        for y in min_y..=max_y {
            let mut line = String::new();
            for x in min_x..=max_x {
                line.push(self.cells[y * self.width + x]);
            }
            lines.push(line);
        }
        lines.join("\n")
    }

    pub fn paste_region(&mut self, dest_x: usize, dest_y: usize, text: &str, transparent_space: bool, bounds: Option<RectBounds>) {
        let rows = crate::ansi::parse_ansi_text(text);
        for (row_idx, row) in rows.into_iter().enumerate() {
            let y = dest_y + row_idx;
            if y >= self.height {
                break;
            }
            for (col_idx, (ch, color)) in row.into_iter().enumerate() {
                let x = dest_x + col_idx;
                if x >= self.width {
                    break;
                }
                if let Some(b) = bounds {
                    if !b.contains(x, y) {
                        continue;
                    }
                }
                if transparent_space && ch == ' ' {
                    continue;
                }
                self.set_colored(x, y, ch, color);
            }
        }
    }

    pub fn ensure_dimensions(&mut self, required_w: usize, required_h: usize) {
        let new_w = self.width.max(required_w);
        let new_h = self.height.max(required_h);
        if new_w > self.width || new_h > self.height {
            self.resize(new_w, new_h, Anchor::TopLeft);
        }
    }

    pub fn paste_region_expanding(
        &mut self,
        dest_x: usize,
        dest_y: usize,
        text: &str,
        transparent_space: bool,
    ) {
        let rows = crate::ansi::parse_ansi_text(text);
        if rows.is_empty() {
            return;
        }
        let text_h = rows.len();
        let text_w = rows.iter().map(|r| r.len()).max().unwrap_or(0);

        self.ensure_dimensions(dest_x + text_w, dest_y + text_h);
        for (row_idx, row) in rows.into_iter().enumerate() {
            let y = dest_y + row_idx;
            if y >= self.height {
                break;
            }
            for (col_idx, (ch, color)) in row.into_iter().enumerate() {
                let x = dest_x + col_idx;
                if x >= self.width {
                    break;
                }
                if transparent_space && ch == ' ' {
                    continue;
                }
                self.set_colored(x, y, ch, color);
            }
        }
    }

    pub fn clear_region(&mut self, bounds: RectBounds) {
        let min_x = bounds.min_x.min(self.width.saturating_sub(1));
        let max_x = bounds.max_x.min(self.width.saturating_sub(1));
        let min_y = bounds.min_y.min(self.height.saturating_sub(1));
        let max_y = bounds.max_y.min(self.height.saturating_sub(1));

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let idx = y * self.width + x;
                self.cells[idx] = ' ';
                self.colors[idx] = ColorId::Default;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoryManager {
    undo_stack: Vec<CanvasGrid>,
    redo_stack: Vec<CanvasGrid>,
    max_history: usize,
}

impl HistoryManager {
    pub fn new(max_history: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_history,
        }
    }

    pub fn push_state(&mut self, current: CanvasGrid) {
        if self.undo_stack.len() >= self.max_history {
            self.undo_stack.remove(0);
        }
        self.undo_stack.push(current);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, current: CanvasGrid) -> Option<CanvasGrid> {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(current);
            Some(prev)
        } else {
            None
        }
    }

    pub fn redo(&mut self, current: CanvasGrid) -> Option<CanvasGrid> {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(current);
            Some(next)
        } else {
            None
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation_and_access() {
        let mut grid = CanvasGrid::new(10, 5);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.get(0, 0), Some(' '));
        assert!(grid.set(2, 3, '#'));
        assert_eq!(grid.get(2, 3), Some('#'));
        assert_eq!(grid.get(10, 5), None);
        grid.clear();
        assert_eq!(grid.get(2, 3), Some(' '));
    }

    #[test]
    fn test_add_and_del_cells() {
        let mut grid = CanvasGrid::new(3, 3);
        grid.set(1, 1, 'X');
        grid.add_cells(Direction::Left, 2);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.get(3, 1), Some('X'));

        grid.del_cells(Direction::Left, 2);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.get(1, 1), Some('X'));
    }

    #[test]
    fn test_crop() {
        let mut grid = CanvasGrid::new(5, 5);
        grid.set(2, 2, 'C');
        grid.crop(RectBounds::new(1, 1, 3, 3));
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.get(1, 1), Some('C'));
    }

    #[test]
    fn test_flood_fill() {
        let mut grid = CanvasGrid::new(4, 4);
        grid.set(0, 0, '.');
        grid.set(0, 1, '.');
        grid.flood_fill(0, 0, '#', ColorId::Default, None);
        assert_eq!(grid.get(0, 0), Some('#'));
        assert_eq!(grid.get(0, 1), Some('#'));
        assert_eq!(grid.get(2, 2), Some(' '));
    }

    #[test]
    fn test_delete_row_and_col() {
        // Test delete row shift up
        let mut grid = CanvasGrid::new(3, 3);
        grid.set_colored(0, 0, 'A', ColorId::Red);
        grid.set_colored(0, 1, 'B', ColorId::Green);
        grid.set_colored(0, 2, 'C', ColorId::Blue);

        grid.delete_row(1, RowShiftMode::ShiftUp);
        assert_eq!(grid.get(0, 0), Some('A'));
        assert_eq!(grid.get_color(0, 0), Some(ColorId::Red));
        assert_eq!(grid.get(0, 1), Some('C'));
        assert_eq!(grid.get_color(0, 1), Some(ColorId::Blue));
        assert_eq!(grid.get(0, 2), Some(' '));

        // Test delete row shift down
        let mut grid2 = CanvasGrid::new(3, 3);
        grid2.set(0, 0, 'A');
        grid2.set(0, 1, 'B');
        grid2.set(0, 2, 'C');
        grid2.delete_row(1, RowShiftMode::ShiftDown);
        assert_eq!(grid2.get(0, 0), Some(' '));
        assert_eq!(grid2.get(0, 1), Some('A'));
        assert_eq!(grid2.get(0, 2), Some('C'));

        // Test delete row clear
        let mut grid3 = CanvasGrid::new(3, 3);
        grid3.set(0, 0, 'A');
        grid3.set(0, 1, 'B');
        grid3.set(0, 2, 'C');
        grid3.delete_row(1, RowShiftMode::Clear);
        assert_eq!(grid3.get(0, 0), Some('A'));
        assert_eq!(grid3.get(0, 1), Some(' '));
        assert_eq!(grid3.get(0, 2), Some('C'));

        // Test delete col shift left
        let mut c_grid = CanvasGrid::new(3, 3);
        c_grid.set(0, 0, '1');
        c_grid.set(1, 0, '2');
        c_grid.set(2, 0, '3');
        c_grid.delete_col(1, ColShiftMode::ShiftLeft);
        assert_eq!(c_grid.get(0, 0), Some('1'));
        assert_eq!(c_grid.get(1, 0), Some('3'));
        assert_eq!(c_grid.get(2, 0), Some(' '));

        // Test delete col shift right
        let mut c_grid2 = CanvasGrid::new(3, 3);
        c_grid2.set(0, 0, '1');
        c_grid2.set(1, 0, '2');
        c_grid2.set(2, 0, '3');
        c_grid2.delete_col(1, ColShiftMode::ShiftRight);
        assert_eq!(c_grid2.get(0, 0), Some(' '));
        assert_eq!(c_grid2.get(1, 0), Some('1'));
        assert_eq!(c_grid2.get(2, 0), Some('3'));

        // Test delete col clear
        let mut c_grid3 = CanvasGrid::new(3, 3);
        c_grid3.set(0, 0, '1');
        c_grid3.set(1, 0, '2');
        c_grid3.set(2, 0, '3');
        c_grid3.delete_col(1, ColShiftMode::Clear);
        assert_eq!(c_grid3.get(0, 0), Some('1'));
        assert_eq!(c_grid3.get(1, 0), Some(' '));
        assert_eq!(c_grid3.get(2, 0), Some('3'));
    }

    #[test]
    fn test_history_undo_redo() {
        let mut history = HistoryManager::new(10);
        let g1 = CanvasGrid::new(5, 5);
        let mut g2 = g1.clone();
        g2.set(0, 0, 'A');

        history.push_state(g1.clone());
        assert!(history.can_undo());
        let undone = history.undo(g2.clone()).unwrap();
        assert_eq!(undone, g1);
        assert!(history.can_redo());
        let redone = history.redo(undone).unwrap();
        assert_eq!(redone, g2);
    }

    #[test]
    fn test_resize_anchors() {
        let mut grid = CanvasGrid::new(2, 2);
        grid.set(0, 0, '1');
        grid.set(1, 0, '2');
        grid.set(0, 1, '3');
        grid.set(1, 1, '4');

        // TopLeft expansion
        let mut g_tl = grid.clone();
        g_tl.resize(4, 4, Anchor::TopLeft);
        assert_eq!(g_tl.get(0, 0), Some('1'));
        assert_eq!(g_tl.get(1, 1), Some('4'));
        assert_eq!(g_tl.get(2, 2), Some(' '));

        // BottomRight expansion
        let mut g_br = grid.clone();
        g_br.resize(4, 4, Anchor::BottomRight);
        assert_eq!(g_br.get(2, 2), Some('1'));
        assert_eq!(g_br.get(3, 3), Some('4'));
        assert_eq!(g_br.get(0, 0), Some(' '));
    }

    #[test]
    fn test_replace_char_and_bounds() {
        let mut grid = CanvasGrid::new(3, 3);
        grid.set(0, 0, 'A');
        grid.set(1, 1, 'A');
        grid.set(2, 2, 'A');

        // Replace only in bounds (0,0) to (1,1)
        grid.replace_char('A', 'B', Some(RectBounds::new(0, 0, 1, 1)));
        assert_eq!(grid.get(0, 0), Some('B'));
        assert_eq!(grid.get(1, 1), Some('B'));
        assert_eq!(grid.get(2, 2), Some('A'));
    }

    #[test]
    fn test_copy_paste_region() {
        let mut grid = CanvasGrid::new(4, 4);
        grid.set(0, 0, 'X');
        grid.set(1, 0, 'Y');
        grid.set(0, 1, 'Z');
        grid.set(1, 1, 'W');

        let text = grid.copy_region(RectBounds::new(0, 0, 1, 1));
        assert_eq!(text, "XY\nZW");

        let mut dest_grid = CanvasGrid::new(4, 4);
        dest_grid.paste_region(2, 2, &text, false, None);
        assert_eq!(dest_grid.get(2, 2), Some('X'));
        assert_eq!(dest_grid.get(3, 2), Some('Y'));
        assert_eq!(dest_grid.get(2, 3), Some('Z'));
        assert_eq!(dest_grid.get(3, 3), Some('W'));
    }

    #[test]
    fn test_string_roundtrip() {
        let ascii_art = "┌──┐\n│  │\n└──┘";
        let grid = CanvasGrid::from_ascii_str(ascii_art);
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);
        let out_str = grid.to_ascii_string();
        assert_eq!(out_str, ascii_art);
    }

    #[test]
    fn test_paste_region_expanding() {
        let mut grid = CanvasGrid::new(2, 2);
        grid.set(0, 0, 'O');

        // Paste 4x3 text starting at (1, 1), requiring grid to expand to 5x4
        let text = "AAAA\nBBBB\nCCCC";
        grid.paste_region_expanding(1, 1, text, false);

        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 4);
        assert_eq!(grid.get(0, 0), Some('O'));
        assert_eq!(grid.get(1, 1), Some('A'));
        assert_eq!(grid.get(4, 1), Some('A'));
        assert_eq!(grid.get(1, 3), Some('C'));
        assert_eq!(grid.get(4, 3), Some('C'));
    }

    #[test]
    fn test_grid_colors() {
        let mut grid = CanvasGrid::new(3, 3);
        assert_eq!(grid.get_color(1, 1), Some(ColorId::Default));

        grid.set_colored(1, 1, '#', ColorId::BrightCyan);
        assert_eq!(grid.get(1, 1), Some('#'));
        assert_eq!(grid.get_color(1, 1), Some(ColorId::BrightCyan));

        // Test crop preserves colors
        let cropped = {
            let mut g = grid.clone();
            g.crop(RectBounds::new(1, 1, 2, 2));
            g
        };
        assert_eq!(cropped.get(0, 0), Some('#'));
        assert_eq!(cropped.get_color(0, 0), Some(ColorId::BrightCyan));

        // Test clear resets colors
        grid.clear();
        assert_eq!(grid.get(1, 1), Some(' '));
        assert_eq!(grid.get_color(1, 1), Some(ColorId::Default));
    }

    #[test]
    fn test_paste_clipped_at_bounds() {
        let mut grid = CanvasGrid::new(4, 4);
        // Paste 3x3 at (2, 2) on a 4x4 canvas: should be clipped at x=3 and y=3
        let text = "ABC\nDEF\nGHI";
        grid.paste_region(2, 2, text, false, None);

        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 4);
        assert_eq!(grid.get(2, 2), Some('A'));
        assert_eq!(grid.get(3, 2), Some('B'));
        assert_eq!(grid.get(2, 3), Some('D'));
        assert_eq!(grid.get(3, 3), Some('E'));
    }

    #[test]
    fn test_ansi_colored_grid_import() {
        let ansi_text = "\x1b[34m[#]\x1b[0m\n\x1b[92mOK!\x1b[0m";
        let grid = CanvasGrid::from_ascii_str(ansi_text);

        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 2);
        assert_eq!(grid.get(0, 0), Some('['));
        assert_eq!(grid.get_color(0, 0), Some(ColorId::Blue));
        assert_eq!(grid.get(1, 0), Some('#'));
        assert_eq!(grid.get_color(1, 0), Some(ColorId::Blue));
        assert_eq!(grid.get(0, 1), Some('O'));
        assert_eq!(grid.get_color(0, 1), Some(ColorId::BrightGreen));
        assert_eq!(grid.get(1, 1), Some('K'));
        assert_eq!(grid.get_color(1, 1), Some(ColorId::BrightGreen));
    }
}
