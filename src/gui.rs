use eframe::egui::{CentralPanel, SidePanel, Color32, TopBottomPanel, ColorImage, TextureOptions, Vec2};
use image::{DynamicImage, ColorType};
use rfd;

pub struct MyApp {
    pub image: Option<DynamicImage>,
    pub error_message: Option<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            image: None,
            error_message: None,
        }
    }
}

fn set_styles(ctx: &eframe::egui::Context) {
    let mut style = (*ctx.style()).clone();
    // Aquí puedes personalizar los estilos, por ejemplo:
    style.visuals.override_text_color = Some(Color32::WHITE);
    ctx.set_style(style);
}

fn show_top_bar(ctx: &eframe::egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.label("Barra Superior - Procesador de Imágenes");
    });
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        set_styles(ctx);
        show_top_bar(ctx);

        SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Controles");
            if ui.button("Subir Imagen").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    match image::ImageReader::open(&path) {
                        Ok(reader) => match reader.decode() {
                            Ok(img) => {
                                if matches!(img.color(), ColorType::L8 | ColorType::La8) {
                                    self.image = Some(img);
                                    self.error_message = None;
                                } else {
                                    self.image = None;
                                    self.error_message = Some("La imagen debe estar en escala de grises. Por favor, sube otra imagen.".to_string());
                                }
                            },
                            Err(_) => {
                                self.error_message = Some("Error al decodificar la imagen.".to_string());
                            }
                        },
                        Err(_) => {
                            self.error_message = Some("Error al abrir el archivo.".to_string());
                        }
                    }
                }
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Imagen");
            ui.horizontal(|ui| {
                // Recuadro para imagen original
                ui.group(|ui| {
                    ui.label("Imagen Original");
                    if let Some(error) = &self.error_message {
                        ui.label(error);
                    } else if let Some(img) = &self.image {
                        let size = [img.width() as usize, img.height() as usize];
                        let color_image = ColorImage::from_rgba_unmultiplied(size, &img.to_rgba8());
                        let texture = ctx.load_texture("image", color_image, TextureOptions::default());
                        ui.image((texture.id(), Vec2::new(500.0, 500.0)));
                    } else {
                        ui.label("No hay imagen cargada.");
                    }
                });

                // Recuadro para imagen procesada (por ahora la misma)
                ui.group(|ui| {
                    ui.label("Imagen Procesada");
                    if let Some(img) = &self.image {
                        let size = [img.width() as usize, img.height() as usize];
                        let color_image = ColorImage::from_rgba_unmultiplied(size, &img.to_rgba8());
                        let texture = ctx.load_texture("processed_image", color_image, TextureOptions::default());
                        ui.image((texture.id(), Vec2::new(500.0, 500.0)));
                    } else {
                        ui.label("No hay imagen procesada.");
                    }
                });
            });
        });
    }
}


