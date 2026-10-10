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
}

impl CanvasGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        Self {
            width,
            height,
            cells: vec![' '; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let index = y * self.width + x;
        self.cells.get(index).copied()
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        let index = y * self.width + x;
        self.cells[index] = c;
        true
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cells.fill(' ');
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
        let lines: Vec<&str> = s.lines().collect();
        if lines.is_empty() {
            return Self::new(60, 30);
        }
        let height = lines.len();
        let width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(1).max(1);

        let mut grid = Self::new(width, height);
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                grid.set(x, y, ch);
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
                for y in 0..self.height {
                    for x in 0..self.width {
                        new_cells[y * new_width + (x + count)] = self.cells[y * self.width + x];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
            }
            Direction::Right => {
                let new_width = self.width + count;
                let mut new_cells = vec![' '; new_width * self.height];
                for y in 0..self.height {
                    for x in 0..self.width {
                        new_cells[y * new_width + x] = self.cells[y * self.width + x];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
            }
            Direction::Top => {
                let new_height = self.height + count;
                let mut new_cells = vec![' '; self.width * new_height];
                let offset = count * self.width;
                new_cells[offset..offset + self.cells.len()].copy_from_slice(&self.cells);
                self.height = new_height;
                self.cells = new_cells;
            }
            Direction::Bottom => {
                let new_height = self.height + count;
                self.cells.resize(self.width * new_height, ' ');
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
                for y in 0..self.height {
                    for x in 0..new_width {
                        new_cells[y * new_width + x] = self.cells[y * self.width + (x + count)];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
            }
            Direction::Right => {
                if count >= self.width {
                    return;
                }
                let new_width = self.width - count;
                let mut new_cells = vec![' '; new_width * self.height];
                for y in 0..self.height {
                    for x in 0..new_width {
                        new_cells[y * new_width + x] = self.cells[y * self.width + x];
                    }
                }
                self.width = new_width;
                self.cells = new_cells;
            }
            Direction::Top => {
                if count >= self.height {
                    return;
                }
                let new_height = self.height - count;
                let start = count * self.width;
                let end = self.cells.len();
                self.cells = self.cells[start..end].to_vec();
                self.height = new_height;
            }
            Direction::Bottom => {
                if count >= self.height {
                    return;
                }
                let new_height = self.height - count;
                self.cells.truncate(self.width * new_height);
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
        for y in 0..new_h {
            for x in 0..new_w {
                let old_idx = (min_y + y) * self.width + (min_x + x);
                new_cells[y * new_w + x] = self.cells[old_idx];
            }
        }
        self.width = new_w;
        self.height = new_h;
        self.cells = new_cells;
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
                if let Some(ch) = self.get(src_x_offset + x, src_y_offset + y) {
                    new_grid.set(dst_x_offset + x, dst_y_offset + y, ch);
                }
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

    pub fn flood_fill(&mut self, start_x: usize, start_y: usize, fill_char: char, bounds: Option<RectBounds>) {
        if start_x >= self.width || start_y >= self.height {
            return;
        }
        if let Some(b) = bounds {
            if !b.contains(start_x, start_y) {
                return;
            }
        }
        let target_char = self.cells[start_y * self.width + start_x];
        if target_char == fill_char {
            return;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start_x, start_y));
        self.cells[start_y * self.width + start_x] = fill_char;

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
                    if self.cells[idx] == target_char {
                        self.cells[idx] = fill_char;
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
        for (row_idx, line) in text.lines().enumerate() {
            let y = dest_y + row_idx;
            if y >= self.height {
                break;
            }
            for (col_idx, ch) in line.chars().enumerate() {
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
                self.set(x, y, ch);
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
                self.cells[y * self.width + x] = ' ';
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
        grid.flood_fill(0, 0, '#', None);
        assert_eq!(grid.get(0, 0), Some('#'));
        assert_eq!(grid.get(0, 1), Some('#'));
        assert_eq!(grid.get(2, 2), Some(' '));
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
}
