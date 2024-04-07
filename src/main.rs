#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] 


mod app;
use app::YasaApp;
use crate::app::Views;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {

    let native_options = eframe::NativeOptions {
        min_window_size: Some([300.0, 200.0].into()),
        initial_window_size: Some([640.0, 400.0].into()),
        transparent: true,
        ..Default::default()
    };
    eframe::run_native(
        "YASA",
        native_options,
        Box::new(|cc| Box::new(app::YasaApp::new(cc))),
    )
}

impl eframe::App for YasaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.view {
            Views::Home => {
                _frame.set_visible(true);
                _frame.set_decorations(true);
                self.home_view(ctx, _frame);
            },
            Views::Screenshot => {
                self.screenshot_view(ctx, _frame);
            },
        }
    }

}
