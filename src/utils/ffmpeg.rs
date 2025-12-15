use std::process::Command;

pub fn extract_frames(
    input_video: &str,
    output_dir: &str,
) {
    Command::new("ffmpeg")
        .args([
            "-i", input_video,
            "-vf", "scale=128:64,format=gray",
            &format!("{}/frame_%04d.png", output_dir),
        ])
        .status()
        .expect("FFmpeg failed");
}

