mod utils;

use std::path::Path;
use utils::frame_writer::process_and_write_frames;
use utils::ffmpeg::extract_frames;


fn main() {
    let frames_dir = Path::new("frames");
    extract_frames("assets/video.mp4", "frames");
    process_and_write_frames(frames_dir);


}
