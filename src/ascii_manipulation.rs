use std::process::exit;
use image::{DynamicImage, GenericImageView, Rgba};
use log::error;
use crate::model_rgb_ascii::Ascii;

fn get_color(pixel: (u32, u32, Rgba<u8>)) -> u8 {
    let vec = pixel.2.0;
    //luminosity method of getting lightness
    (vec[0] as f32 * 0.3 + vec[1] as f32 * 0.59 + vec[2] as f32 * 0.11) as u8
}

fn to_ascii(char_map: String, image: DynamicImage) -> Vec<Vec<Ascii>> {
    let l = char_map.len() as f32;
    let mut str: Vec<Ascii> = Vec::new();
    let mut out: Vec<Vec<Ascii>> = Vec::new();

    for pixel in image.pixels() {
        let ch = char_map.as_bytes()[((get_color(pixel) as f32-1.0)/255f32 * l) as usize];
        //fixme: might break with non-ASCII char_map (ie braille chars, possibly)
        str.push(Ascii::new(ch, pixel.2[0], pixel.2[1], pixel.2[2]));

        if pixel.0 == image.width()-1 {
            out.push(str);
            str = Vec::new();
        }
    }
    out
}

pub fn to_simple_ascii(image: DynamicImage) -> Vec<Vec<Ascii>> {
    to_ascii(" .:-=+*#%@".to_owned(), image)
}

pub fn to_complex_ascii(image: DynamicImage) -> Vec<Vec<Ascii>> {
    to_ascii(" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".to_owned(), image)
}

pub fn to_braille_ascii(image: DynamicImage) -> Vec<Vec<Ascii>> {
    //todo: figure out braille symbols
    vec![vec![Ascii::new(0, 0, 0, 0)]]
}

pub fn to_custom_ascii(char_map: String, image: DynamicImage) -> Vec<Vec<Ascii>> {
    if char_map.is_empty() {
        error!("Custom map can not be empty!");
        exit(1);
    }
    to_ascii(char_map, image)
}