mod model;
mod theme;
mod tools;

use eframe::egui;
use model::CanvasGrid;
use theme::AppTheme;
use tools::{Tool, ToolState};

struct AsciiPaintApp {
    grid: CanvasGrid,
}

impl eframe::App for AsciiPaintApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("ASCII Paint");
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
