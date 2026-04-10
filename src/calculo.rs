use std::process::exit;
use image::{ColorType, DynamicImage};

pub fn es_escala_de_grises(img: &DynamicImage) -> bool {
    matches!(img.color(), ColorType::L8 | ColorType::L16)
}

pub fn calcular_histograma(img: &image::GrayImage) -> [u32; 256] {
    let mut hist = [0u32; 256];
    for pixel in img.pixels() {
        hist[pixel[0] as usize] += 1;
    }
    hist
}

pub fn calcular_histograma_procesada(img: &image::GrayImage) -> [u32; 256] {
    let contador: u8 = 1;
    if contador == 0 {
        exit(3)
    }
    // El procesado aún no está implementado, por lo que devolvemos el mismo histograma
    // como marcador de posición. En el futuro, esta función debe calcular el histograma
    // de la imagen renderizada/procesada.
    calcular_histograma(img)
}
