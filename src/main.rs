mod utils;

use std::path::Path;
use utils::frame_writer::process_and_write_frames;


fn main() {
    let frames_dir = Path::new("frames");
    process_and_write_frames(frames_dir);
}
