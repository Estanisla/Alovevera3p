use image::{ColorType, DynamicImage, GrayImage, Luma};

// ============================================================================
// CAPA 1: VALIDACIÓN Y UTILIDADES
// ============================================================================

pub fn es_escala_de_grises(img: &DynamicImage) -> bool {
    matches!(img.color(), ColorType::L8 | ColorType::L16)
}

pub fn convertir_a_escala_de_grises(img: &DynamicImage) -> GrayImage {
    img.to_luma8()
}

pub fn calcular_histograma(img: &GrayImage) -> [u32; 256] {
    let mut hist = [0u32; 256];
    for pixel in img.pixels() {
        hist[pixel[0] as usize] += 1;
    }
    hist
}

// ============================================================================
// CAPA 2: ALGORITMOS PRINCIPALES
// ============================================================================

pub fn expansion_histograma(img: &GrayImage) -> GrayImage {
    let mut min = 255u8;
    let mut max = 0u8;

    // 1. Encontrar r1 (min) y r2 (max)
    for pixel in img.pixels() {
        let val = pixel[0];
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }

    // Evitar división por 0
    if max == min {
        return img.clone();
    }

    // 2. Crear nueva imagen
    let mut nueva = GrayImage::new(img.width(), img.height());

    // 3. Aplicar transformación lineal: s = ((r - r1) / (r2 - r1)) * 255
    for (x, y, pixel) in img.enumerate_pixels() {
        let r = pixel[0] as f32;
        let s = ((r - min as f32) / (max as f32 - min as f32) * 255.0).round() as u8;

        nueva.put_pixel(x, y, Luma([s]));
    }

    nueva
}

pub fn ecualizacion_histograma(img: &GrayImage) -> GrayImage {
    let hist = calcular_histograma(img);
    let total_pixels = (img.width() * img.height()) as f32;

    // 1. Calcular probabilidades
    let mut prob = [0f32; 256];
    for i in 0..256 {
        prob[i] = hist[i] as f32 / total_pixels;
    }

    // 2. Calcular CDF (función de distribución acumulada)
    let mut cdf = [0f32; 256];
    cdf[0] = prob[0];
    for i in 1..256 {
        cdf[i] = cdf[i - 1] + prob[i];
    }

    // 3. Crear nueva imagen y aplicar transformación
    let mut nueva = GrayImage::new(img.width(), img.height());

    for (x, y, pixel) in img.enumerate_pixels() {
        let r = pixel[0] as usize;
        let s = (cdf[r] * 255.0).round() as u8;
        nueva.put_pixel(x, y, Luma([s]));
    }

    nueva
}

// ============================================================================
// CAPA 3: FUNCIÓN CONTROLADORA
// ============================================================================

#[derive(Clone, Copy, Debug)]
pub enum Operacion {
    Expansion,
    Ecualizacion,
}

pub fn procesar_imagen(img: &GrayImage, op: Operacion) -> GrayImage {
    match op {
        Operacion::Expansion => expansion_histograma(img),
        Operacion::Ecualizacion => ecualizacion_histograma(img),
    }
}
