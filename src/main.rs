mod utils;

use std::path::Path;
use utils::frame_writer::process_and_write_frames;


fn main() {
    let frames_dir = Path::new("frames");
    let output_dir = Path::new("output");
    let output_file = output_dir.join("frames.raw");

    std::fs::create_dir_all(output_dir)
        .expect("No se pudo crear output/");

    process_and_write_frames(frames_dir, &output_file);
}
