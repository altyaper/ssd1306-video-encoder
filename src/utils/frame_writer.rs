use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::utils::image_processing::{load_image_to_binary, pack_frame};
use crate::utils::rle::rle_encode;

/// Processes all PNG frames in the given directory, encodes them, and writes to the output file.
pub fn process_and_write_frames(frames_dir: &Path, output_file: &Path) {
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

    let mut output = File::create(output_file)
        .expect("No se pudo crear frames.raw");

    println!("Procesando {} frames...", entries.len());

    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();
        let binary = load_image_to_binary(&path);
        let frame = pack_frame(&binary);
        let rle = rle_encode(&frame);
        output
            .write_all(&rle)
            .expect("Error escribiendo frame");
        if i % 50 == 0 {
            println!("Procesado frame {}", i);
        }
    }

    println!(
        "Listo ✅ {} frames escritos en {:?}",
        entries.len(),
        output_file
    );
}
