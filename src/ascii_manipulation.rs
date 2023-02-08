use image::{DynamicImage, GenericImageView, Rgba};

//todo: consider how to take care of the a channel => do we want to render that as background?
fn get_color(pixel: (u32, u32, Rgba<u8>)) -> u8 {
    let vec = pixel.2.0;
    //luminosity method of getting lightness
    (vec[0] as f32 * 0.3 + vec[1] as f32 * 0.59 + vec[2] as f32 * 0.11) as u8
}

fn to_ascii(char_map: String, image: DynamicImage) -> Vec<String> {
    //todo: add color support
    let l = char_map.len() as f32;
    let mut str = String::new();
    let mut out: Vec<String> = Vec::new();

    for pixel in image.pixels() {
        let ch = char_map.as_bytes()[((get_color(pixel) as f32-1.0)/255f32 * l) as usize];  //fixme: might break with non-ASCII char_map (ie braille chars, possibly)
        str.push(char::from(ch));   //fixme: this is finicky and also very unsafe

        if pixel.0 == image.width()-1 {
            out.push(str);
            str = String::new();
        }
    }
    out
}

pub fn to_simple_ascii(image: DynamicImage) -> Vec<String> {
    to_ascii(" .:-=+*#%@".to_owned(), image)
}

pub fn to_complex_ascii(image: DynamicImage) -> Vec<String> {
    to_ascii(" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".to_owned(), image)
}

pub fn to_braille_ascii(image: DynamicImage) -> Vec<String> {
    //todo: figure out braille symbols
    vec!["not implemented".to_owned()]
}

pub fn to_custom_ascii(char_map: String, image: DynamicImage) -> Vec<String> {
    //todo: this
    vec!["not implemented".to_owned()]
}
