use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::utils::image_processing::{load_image_to_binary, pack_frame};
use crate::utils::rle::rle_encode;

/// Processes all PNG frames in the given directory, encodes them, and writes to the output file.
pub fn process_and_write_frames(frames_dir: &Path) {

    // Read the directory and filter for PNG files
    let mut entries: Vec<_> = fs::read_dir(frames_dir)
        .expect("Could not read frames/")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "png")
                .unwrap_or(false)
        })
        .collect();

    // Sort entries by file path
    entries.sort_by_key(|e| e.path());

    // Create output files for raw and RLE encoded frames
    let mut raw_output = File::create("output/frames.raw")
        .expect("Could not create frames.raw");

    let mut rle_output = File::create("output/frames.rle")
        .expect("Could not create frames.rle");

    println!("Processing {} frames...", entries.len());

    // Process each frame
    for (i, entry) in entries.iter().enumerate() {
        let path = entry.path();

        // 1. Convert PNG to binary
        let binary = load_image_to_binary(&path);

        // 2. Convert binary to framebuffer
        let frame = pack_frame(&binary);

        // 3. Write RAW frame to output
        raw_output
            .write_all(&frame)
            .expect("Error writing RAW frame");

        // 4. Perform RLE encoding
        let rle = rle_encode(&frame);

        // 5. Write RLE encoded frame to output
        rle_output
            .write_all(&rle)
            .expect("Error writing RLE frame");

        // Print progress every 50 frames
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
