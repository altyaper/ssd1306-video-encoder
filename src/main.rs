mod utils;

use std::path::Path;
use utils::frame_writer::process_and_write_frames;
use utils::ffmpeg::extract_frames;

/* 
** Entry point of the application
** Extracts frames from a video file, all the frames are saved in the "frames/" directory
** Then we generate a raw which represents the processed frames
** Then we generate a RLE compressed version of the raw file
** Finally we generate a C header file containing the RLE compressed data using heatshrink algorithm
**
**
** VIDEO -> FRAMES (PNG images) -> RAW -> RLE -> HEATSHRINK C HEADER
**
** Notes: RLE is a simple compression algorithm that replaces consecutive identical values with a single value and a count
** Heatshrink is a more advanced compression algorithm that is particularly effective for data with repeating patterns
** 
*/

fn main() {
    // Ensure frames directory exists
    let frames_dir = Path::new("frames");
    std::fs::create_dir_all(frames_dir).expect("Failed to create frames directory");

    let output_dir = Path::new("output");
    std::fs::create_dir_all(output_dir).expect("Failed to create frames directory");

    // extract frames from video, save PNG images in frames/ directory
    extract_frames("assets/video.mp4", "frames");

    // process frames and write raw and RLE files
    process_and_write_frames(frames_dir);
}
