
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Ascii {
    pub char: char,
    pub rgb: RGB,
}


impl Ascii {
    fn _new(char: char, r: u8, g: u8, b: u8) -> Self {
        Ascii {
            char,
            rgb: RGB {
                r,
                g,
                b,
            },
        }
    }

    pub fn new(ch: u8, r: u8, g: u8, b: u8) -> Self {
        Self::_new(char::from(ch), r, g, b)
    }
}