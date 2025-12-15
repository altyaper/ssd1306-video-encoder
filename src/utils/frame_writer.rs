use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::utils::image_processing::{load_image_to_binary, pack_frame};
use crate::utils::rle::rle_encode;

/// Processes all PNG frames in the given directory, encodes them, and writes to the output file.
pub fn process_and_write_frames(frames_dir: &Path) {

    let mut entries: Vec<_> = fs::read_dir(frames_dir)
        .expect("No se pudo leer frames/")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "png")
                .unwrap_or(false)
        })
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut raw_output = File::create("output/frames.raw")
    .expect("No se pudo crear frames.raw");

    let mut rle_output = File::create("output/frames.rle")
    .expect("No se pudo crear frames.rle");

    println!("Procesando {} frames...", entries.len());

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();

        // 1. PNG → binario
        let binary = load_image_to_binary(&path);

        // 2. binario → framebuffer
        let frame = pack_frame(&binary);

        // 3. escribir RAW
        raw_output
            .write_all(&frame)
            .expect("Error escribiendo frame RAW");

        // 4. RLE
        let rle = rle_encode(&frame);

        // 5. escribir RLE
        rle_output
            .write_all(&rle)
            .expect("Error escribiendo frame RLE");

        if i % 50 == 0 {
            println!(
                "Frame {} → RAW: {} bytes | RLE: {} bytes",
                i,
                frame.len(),
                rle.len()
            );
        }
    }
}
