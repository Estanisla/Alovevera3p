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
