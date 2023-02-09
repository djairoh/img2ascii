//! This module contains the Ascii struct.

/// This struct represents one character + rgb colour.
///
/// attributes:
/// char: char - the character associated with this ASCII value
/// rgb: [u8; 3] - vec of 3 u8 values representing red, green, and blue respectively
/// col_depthL u8 - the depth of the rgb values, calculated through the luminosity method
pub struct Ascii {
    pub char: char,
    pub rgb: [u8; 3],
    pub col_depth: u8,
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
            col_depth: (r as f32 * 0.3 + g as f32 * 0.59 + b as f32 * 0.11) as u8,
        }
    }

    /// This function creates a new Ascii instance.
    ///
    /// Arguments:
    /// ch: char - the character
    /// r: u8 - Red colour value
    /// g: u8 - Green colour value
    /// b: u8 - Blue colour value
    ///
    /// returns:
    /// self
    pub fn new_with_char(ch: char, r: u8, g: u8, b: u8) -> Self {
        Ascii {
            char: ch,
            rgb: [r, g, b],
            col_depth: (r as f32 * 0.3 + g as f32 * 0.59 + b as f32 * 0.11) as u8,
        }
    }
}