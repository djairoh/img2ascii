use std::process::exit;
use image::{DynamicImage, GenericImageView, imageops, Rgba};
use log::error;
use crate::model_rgb_ascii::Ascii;

const BRAILLE_TABLE: [[u8; 2]; 4] = [[1u8, 8u8], [2u8, 16u8], [4u8, 32u8], [64u8, 128u8]];

fn get_color(pixel: Rgba<u8>) -> u8 {
    //luminosity method of getting lightness
    (pixel[0] as f32 * 0.3 + pixel[1] as f32 * 0.59 + pixel[2] as f32 * 0.11) as u8
}

fn to_ascii(char_map: String, image: DynamicImage) -> Vec<Vec<Ascii>> {
    let l = char_map.len() as f32;
    let mut str: Vec<Ascii> = Vec::new();
    let mut out: Vec<Vec<Ascii>> = Vec::new();

    for pixel in image.pixels() {
        let ch = char_map.as_bytes()[((get_color(pixel.2) as f32-1.0)/255f32 * l) as usize];
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

pub fn to_custom_ascii(char_map: String, image: DynamicImage) -> Vec<Vec<Ascii>> {
    if char_map.is_empty() {
        error!("Custom map can not be empty!");
        exit(1);
    }
    to_ascii(char_map, image)
}

pub fn to_braille_ascii(mut image: DynamicImage, threshold: u8) -> Vec<Vec<Ascii>> {
    let mut str: Vec<Ascii> = Vec::new();
    let mut out: Vec<Vec<Ascii>> = Vec::new();

    //you know your code is good when you have a quadruple nested for loop
    for y in (0..image.height()).step_by(4) {
        for x in (0..image.width()).step_by(2) {
            let (mut r, mut g, mut b) = (0u16, 0u16, 0u16);
            let mut braille_value: u32 = 10240;
            for nx in 0..2 {
                for ny in 0..4 {
                    let pixel = image.get_pixel(x+nx, y+ny);
                    r = r+pixel[0] as u16;
                    g = g+pixel[1] as u16;
                    b = b+pixel[2] as u16;
                    if get_color(pixel) >= threshold {
                        braille_value += BRAILLE_TABLE[ny as usize][nx as usize] as u32;
                    }
                }
            }
            str.push(Ascii { char: char::from_u32(braille_value).unwrap(), rgb: [(r/8) as u8, (g/8) as u8, (b/8) as u8] });
            }
        out.push(str);
        str = Vec::new();
    }
    out
}