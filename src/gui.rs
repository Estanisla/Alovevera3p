use crate::calculo::{calcular_histograma, calcular_histograma_procesada};
use eframe::egui::{CentralPanel, SidePanel, Color32, Pos2, Rect, Sense, Stroke, StrokeKind, TopBottomPanel, ColorImage, TextureOptions, Vec2};
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

fn mostrar_resumen_histograma(ui: &mut eframe::egui::Ui, hist: &[u32; 256]) {
    let total_pixels: u32 = hist.iter().sum();
    let max_occurrences = hist.iter().copied().max().unwrap_or(0);

    ui.label(format!("Total de píxeles: {}", total_pixels));
    ui.label(format!("Máximo de ocurrencias en un nivel: {}", max_occurrences));
    ui.label("Cuentas de intensidades seleccionadas:");
    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            for i in 0..8 {
                let idx = i * 32;
                ui.label(format!("{:03}: {}", idx, hist[idx]));
            }
        });
        ui.add_space(20.0);
        ui.vertical(|ui| {
            for i in 0..8 {
                let idx = i * 32 + 16;
                ui.label(format!("{:03}: {}", idx, hist[idx]));
            }
        });
    });
}

fn mostrar_histograma(ui: &mut eframe::egui::Ui, hist: &[u32; 256]) {
    let desired_size = Vec2::new(500.0, 180.0);
    let (rect, _response) = ui.allocate_exact_size(desired_size, Sense::hover());
    let painter = ui.painter();
    let max_value = hist.iter().copied().max().unwrap_or(1) as f32;
    let inner = rect.shrink(8.0);
    let width = inner.width().max(1.0);
    let height = inner.height().max(1.0);
    let bar_width = width / 256.0;

    for (i, &value) in hist.iter().enumerate() {
        let x = inner.left() + i as f32 * bar_width;
        let bar_height = if max_value > 0.0 {
            height * (value as f32 / max_value)
        } else {
            0.0
        };
        let color = if i < 85 {
            Color32::from_rgb(90, 160, 255)
        } else if i < 170 {
            Color32::from_rgb(120, 255, 165)
        } else {
            Color32::from_rgb(255, 180, 120)
        };
        let bar_rect = Rect::from_min_max(
            Pos2::new(x, inner.bottom() - bar_height),
            Pos2::new(x + bar_width.max(1.0), inner.bottom()),
        );
        painter.rect_filled(bar_rect, 0.0, color);
    }

    painter.rect_stroke(inner, 0.0, Stroke::new(1.0, Color32::WHITE), StrokeKind::Outside);
    ui.horizontal(|ui| {
        ui.label("0");
        let spacer = (inner.width() - 24.0).max(0.0);
        ui.add_space(spacer);
        ui.label("255");
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
            ui.vertical(|ui| {
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

                if let Some(img) = &self.image {
                    let hist = calcular_histograma(&img.to_luma8());
                    let processed_hist = calcular_histograma_procesada(&img.to_luma8());

                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
                        ui.group(|ui| {
                            ui.label("Histograma Imagen Original");
                            mostrar_resumen_histograma(ui, &hist);
                            mostrar_histograma(ui, &hist);
                        });

                        ui.group(|ui| {
                            ui.label("Histograma Imagen Procesada");
                            ui.colored_label(Color32::YELLOW, "Procesado no implementado. Se muestra el histograma original como marcador de posición.");
                            mostrar_resumen_histograma(ui, &processed_hist);
                            mostrar_histograma(ui, &processed_hist);
                        });
                    });
                }
            });
        });
    }
}


