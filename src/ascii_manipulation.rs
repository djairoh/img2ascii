//! This module contains all functions related to converting images to ASCII.
use std::process::exit;
use image::{DynamicImage, GenericImageView, Rgba};
use log::error;
use crate::model_rgb_ascii::Ascii;

/// This constant is used to calculate braille values.
const BRAILLE_TABLE: [[u8; 2]; 4] = [[1u8, 8u8], [2u8, 16u8], [4u8, 32u8], [64u8, 128u8]];

/// This function returns the colour depth of an input pixel.
///
/// It does so using the luminosity method
/// (0.3*r + 0.59*g + 0.11*b)
///
/// arguments:
/// pixel: Rgba<u8> - pixel to return colour from
///
/// returns:
/// u8 representing colour depth of pixel
fn get_color(pixel: Rgba<u8>) -> u8 {
    //luminosity method of getting lightness
    (pixel[0] as f32 * 0.3 + pixel[1] as f32 * 0.59 + pixel[2] as f32 * 0.11) as u8
}

/// This function converts a given image into ASCII.
///
/// arguments:
/// char_map: String - the characters to convert the image into
/// image: DynamicImage - the image to convert into char_map's characters
///
/// returns:
/// Vec<Vec<Ascii>> containing the converted image
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

/// This wrapper function converts an image to standard ASCII.
///
/// It uses the char_map " .:-=+*#%@"
///
/// arguments:
/// image: DynamicImage - the image to convert
///
/// returns:
/// Vec<Vec<Ascii>> containing the converted image
pub fn to_simple_ascii(image: DynamicImage) -> Vec<Vec<Ascii>> {
    to_ascii(" .:-=+*#%@".to_owned(), image)
}

/// This wrapper function converts an image to extended ASCII.
///
/// It uses the char_map " .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$"
///
/// arguments:
/// image: DynamicImage - the image to convert
///
/// returns:
/// Vec<Vec<Ascii>> containing the converted image
pub fn to_complex_ascii(image: DynamicImage) -> Vec<Vec<Ascii>> {
    to_ascii(" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".to_owned(), image)
}

/// This wrapper function converts an image to custom ASCII.
///
/// arguments:
/// char_map: String - the custom character map to convert image into
/// image: DynamicImage - the image to convert
///
/// returns:
/// Vec<Vec<Ascii>> containing the converted image
pub fn to_custom_ascii(char_map: String, image: DynamicImage) -> Vec<Vec<Ascii>> {
    if char_map.is_empty() {
        error!("Custom map can not be empty!");
        exit(1);
    }
    to_ascii(char_map, image)
}

/// This function converts an image to braille ASCII.
///
/// arguments:
/// image: DynamicImage - the image to convert
///
/// returns:
/// Vec<Vec<Ascii>> containing the converted image
pub fn to_braille_ascii(image: DynamicImage, threshold: u8) -> Vec<Vec<Ascii>> {
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
            str.push(Ascii::new_with_char(char::from_u32(braille_value).unwrap(), (r/8) as u8, (g/8) as u8,  (b/8) as u8));
            }
        out.push(str);
        str = Vec::new();
    }
    out
}