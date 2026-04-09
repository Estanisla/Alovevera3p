use eframe::egui;
use image::{ColorType, DynamicImage, ImageReader};

fn es_escala_de_grises(img: &DynamicImage) -> bool {
    matches!(img.color(), ColorType::L8 | ColorType::L16)
}

fn calcular_histograma(img: &image::GrayImage) -> [u32; 256] {
    let mut hist = [0u32; 256];
    for pixel in img.pixels() {
        hist[pixel[0] as usize] += 1;
    }
    hist
}

fn main() -> eframe::Result {
    // Cargar imagen como DynamicImage
    let imagen = ImageReader::open("C:\\Users\\Estanislao\\Desktop\\Rust\\Alovevera\\src\\yang.jpg")
        .expect("webon pasa imagen")
        .decode()
        .expect("tarao de mrd tu imagen no sirve kgda");

    // Si la imagen ya está en escala de grises, convertir a GrayImage y calcular histograma
    if es_escala_de_grises(&imagen) {
        // to_luma8() devuelve `image::GrayImage`
        let gray = imagen.to_luma8();
        let calculo = calcular_histograma(&gray);
        // evitar warning de variable no usada si no hacemos nada con el histograma ahora
        println!("{:?}", calculo)
    } else {println!("pendejo no es blanco y negro")};

    let options = eframe::NativeOptions {
      viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
      ..Default::default()  
    };  
    eframe::run_native("DEMO", options, Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}
struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            name: "Semicolon".to_owned(),
            age: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Name: {}", self.name));
            ui.label(format!("Age: {}", self.age));
        });
    }
}