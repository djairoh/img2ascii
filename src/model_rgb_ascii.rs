pub struct Ascii {
    pub char: char,
    pub rgb: [u8; 3],
}

impl Ascii {
    pub fn new(ch: u8, r: u8, g: u8, b: u8) -> Self {
        Ascii {
            char: char::from(ch),
            rgb: [r, g, b],
        }
    }
}