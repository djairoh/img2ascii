//! This module contains the Ascii struct.

/// This struct represents one character + rgb colour.
pub struct Ascii {
    pub char: char,
    pub rgb: [u8; 3],
}

impl Ascii {
    /// This function creates a new Ascii instance.
    ///
    /// Arguments:
    /// ch: u8 - the character
    /// r: u8 - Red colour value
    /// g: u8 - Green colour value
    /// b: u8 - Blue colour value
    ///
    /// returns:
    /// self
    pub fn new(ch: u8, r: u8, g: u8, b: u8) -> Self {
        Ascii {
            char: char::from(ch),
            rgb: [r, g, b],
        }
    }
}