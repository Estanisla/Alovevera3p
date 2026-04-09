mod calculo;
mod gui;

use eframe::egui;
use gui::MyApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default().with_inner_size([1980.0, 850.0]),
      ..Default::default()  
    };  
    eframe::run_native("Procesador de Imágenes", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}