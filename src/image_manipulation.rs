use termion::terminal_size;
use log::{debug, error, trace};
use std::process::exit;
use std::path::PathBuf;
use image::DynamicImage;
use image::imageops::FilterType;

fn get_terminal_size() -> (u32, u32) {
    let size = terminal_size();
    match size {
        Ok(size) => {
            (size.0 as u32, size.1 as u32)
        }
        Err(e) => {
            error!("Failed to get terminal size: {}", e.to_string());
            exit(1);
        }
    }
}

pub fn resize_image(img: DynamicImage, full: bool, opt_w: Option<u32>) -> DynamicImage {
    let (mut w, mut h) = (1,1);
    if full {
        w = get_terminal_size().0;
        h = (img.height() as f32 * w as f32 / img.width() as f32 * 0.5) as u32;
    } else if let Some(act_w) = opt_w {
        w = act_w;
        h = (img.height() as f32 * w as f32 / img.width() as f32 * 0.5) as u32;
    } else {
        let (max_w, max_h) = get_terminal_size();
        if max_h*max_w/2 > img.height()*img.width() {
            h = max_h;
            w = (img.width() as f32 * h as f32 / img.height() as f32 * 2.0) as u32;
        } else {
            w = max_w;
            h = (img.height() as f32 * w as f32 / img.height() as f32 * 0.5) as u32;
        }
    }
    debug!("Resizing image to (w|h): {} | {}", w, h);
    img.resize_exact(w, h, FilterType::CatmullRom)
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