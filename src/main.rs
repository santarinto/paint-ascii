use eframe::egui;

struct CanvasGrid {
    width: usize,
    height: usize,
    cells: Vec<char>,
}

struct AsciiPaintApp {
    grid: CanvasGrid,
}

impl eframe::App for AsciiPaintApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Hello from ASCII Paint");

        let cell_w = 12.0;
        let cell_h = 20.0;
        let canvas_size = egui::vec2(
            self.grid.width as f32 * cell_w,
            self.grid.height as f32 * cell_h,
        );

        let (response, painter) = ui.allocate_painter(
            canvas_size,
            egui::Sense::click_and_drag(),
        );

        painter.rect_filled(
            response.rect, 
            0.0, 
            egui::Color32::from_rgb(30, 30 ,35),
        );
    }
}

impl CanvasGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height, 
            cells: vec![ ' '; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<char> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = y * self.width + x;

        self.cells.get(index).copied()
    }

    fn set(&mut self, x: usize, y: usize, c: char) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }

        let index = y * self.width + x;
        self.cells[index] = c;
        true
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "ASCII Paint",
        native_options,
        Box::new(|_cc| Ok(Box::new(AsciiPaintApp {
            grid: CanvasGrid::new(60, 30),
        }))),
    )
}
