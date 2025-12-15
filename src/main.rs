mod utils;

use utils::ffmpeg::extract_frames;

fn main() {
    let output_dir = "frames";
    std::fs::create_dir_all(output_dir)
    .expect("No se pudo crear el directorio de salida");

    extract_frames(
        "assets/video.mov", // input
        output_dir,           // output directory
    );

    println!("Frames extraídos correctamente");
}