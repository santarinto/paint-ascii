use eframe::egui;

struct AsciiPaintApp {
    
}

impl eframe::App for AsciiPaintApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Hello from ASCII Paint");
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    
    eframe::run_native(
        "ASCII Paint",
        native_options,
        Box::new(|_cc| Ok(Box::new(AsciiPaintApp {}))),
    )
}
