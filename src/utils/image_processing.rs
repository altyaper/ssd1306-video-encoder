use image::{GenericImageView, imageops::FilterType};
use std::path::Path;

/// Loads a PNG image and converts it to a 128x64 binary matrix
pub fn load_image_to_binary(path: &Path) -> [[u8; 128]; 64] {
    let img = image::open(path)
        .unwrap_or_else(|_| panic!("No se pudo abrir {:?}", path));

    let img = img.resize_exact(128, 64, FilterType::Nearest);

    let mut binary = [[0u8; 128]; 64];

    for y in 0..64 {
        for x in 0..128 {
            let pixel = img.get_pixel(x as u32, y as u32);
            let [r, g, b, _] = pixel.0;

            let luminance = (r as u16 + g as u16 + b as u16) / 3;
            binary[y][x] = if luminance > 128 { 1 } else { 0 };
        }
    }

    binary
}

/// Packs the binary image into a 1024-byte framebuffer (row-major, MSB-first)
pub fn pack_frame(image: &[[u8; 128]; 64]) -> [u8; 1024] {
    let mut frame = [0u8; 1024];

    for y in 0..64 {
        for x in 0..128 {
            if image[y][x] == 0 {
                continue;
            }

            let byte_index = y * 16 + (x / 8);
            let bit_index = 7 - (x % 8);

            frame[byte_index] |= 1 << bit_index;
        }
    }

    frame
}