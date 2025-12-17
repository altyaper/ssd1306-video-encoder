/// Extracts frames from a video file and saves them as grayscale PNG images.
///
/// This function uses FFmpeg to process the input video, scaling each frame to 128x64 pixels
/// and converting it to grayscale format. Extracted frames are saved sequentially with
/// zero-padded filenames (frame_0001.png, frame_0002.png, etc.).
///
/// # Arguments
///
/// * `input_video` - Path to the input video file
/// * `output_dir` - Directory path where extracted frames will be saved
///
/// # Panics
///
/// Panics if FFmpeg is not installed, not found in PATH, or if the extraction process fails.
///
/// # Example
///
/// ```no_run
/// extract_frames("video.mp4", "./output_frames");
/// ```
pub fn extract_frames(
    input_video: &str,
    output_dir: &str,
) {
    // Check if FFmpeg is installed
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .expect("FFmpeg is not installed or not found in PATH. Please install FFmpeg to use this function.");
    Command::new("ffmpeg")
        .args([
            "-i", input_video,
            "-vf", "scale=128:64,format=gray",
            &format!("{}/frame_%04d.png", output_dir),
        ])
        .status()
        .expect("FFmpeg failed");
}

