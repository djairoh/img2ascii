use std::cmp::min;
use termion::terminal_size;
use log::error;
use std::process::exit;
use std::path::PathBuf;
use image::DynamicImage;

fn get_terminal_size() -> u32 {
    let size = terminal_size();
    match size {
        Ok(size) => {
            size.0 as u32
        }
        Err(e) => {
            error!("Failed to get terminal size: {}", e.to_string());
            exit(1);
        }
    }
}

pub fn get_size(w: Option<u16>, img_w: u32) -> u16 {
    if None == w {
        min(get_terminal_size(), img_w) as u16
    } else {
        w.unwrap()
    }
}

pub fn open_image(path: PathBuf) -> DynamicImage {
    let img = image::open(path);
    match img {
        Ok(img) => img,
        Err(e) => {
            error!("Failed to open image: {}", e.to_string());
            exit(1)
        }
    }
}
