use std::ffi::OsString;
use std::path::PathBuf;
use std::process::exit;
use clap::Parser;
use log::{debug, error};

/// Convert an image to ASCII art and print it to the terminal.
///
/// Configuration is done using flags.
#[derive(Parser)]
pub struct Cli {
    /// Use a larger range of characters
    ///
    /// Takes priority over '--map'
    #[arg(short = 'c', long)]
    pub complex: bool,
    /// Display ASCII art in full colour
    #[arg(short = 'C', long)]
    pub colour: bool,
    /// Use braille characters instead of ASCII
    ///
    /// Takes priority over '--complex' and '--map'
    #[arg(short = 'b', long)]
    pub braille: bool,
    /// Print debugging information
    #[arg(short = 'd', long)]
    pub debug: bool,
    /// use the full width of the terminal
    ///
    /// Takes priority over '--width'
    #[arg(short = 'f', long)]
    pub full: bool,
    /// use a custom character map for output
    ///
    /// The character map is interpreted from dark(first character) to light(last character).
    /// The map can be any size, but at least 2 characters is recommended.
    #[arg(short = 'M', long)]
    pub map: Option<String>,
    /// Image path
    pub image: PathBuf,
    /// Set the width of the output, instead of using terminal width
    #[arg(short = 'w', long)]
    pub width: Option<u32>,
    /// Save the output to a file, instead of printing to terminal
    ///
    /// Incompatible with '--colour' and '--braille'
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,
}


impl Cli {
    pub fn debug_print(&self) {
        debug!("complex: {}", self.complex);
        debug!("colour: {}", self.colour);
        debug!("braille: {}", self.braille);
        debug!("debug: {}", self.debug);
        debug!("full: {}", self.full);
        if let Some(map) = self.map.clone() {
            debug!("map: \"{}\"", map);
        } else {
            debug!("map: None");
        }
        debug!("width: {}", self.width.unwrap_or(u32::MAX));
        debug!("image: {}", self.image.display());
        if let Some(output) = self.output.clone() {
            debug!("output: {}", output.display());
        } else {
            debug!("output: None");
        }
    }

    pub fn validate(&self) {
        if !file_exists(self.image.clone()) {
            error!("Input file \"{}\" does not exist!", self.image.display());
            exit(1);
        }

        if !is_image(self.image.clone()) {
            error!("Input file \"{}\" is not an image!", self.image.display());
            exit(1);
        }

        if let Some(output) = self.output.clone() {
            if file_exists(output.clone()) {
                error!("Output file \"{}\" already exists!", output.display());
                exit(1);
            }
        }
    }

    pub fn init(&self) {
        if self.debug {std::env::set_var("RUST_LOG", "trace")}
        env_logger::init();
        self.debug_print();
    }
}

/// This function checks if a given file is an image
///
/// arguments:
/// path: PathBuf - path to the file to check
///
/// returns:
/// boolean indicating if file is an image or not
fn is_image(path: PathBuf) -> bool {
    let ext = path.extension();
    match ext {
        Some(ext) => {
            vec![OsString::from("png"),
                 OsString::from("jpg"),
                 OsString::from("jpeg"),
                 OsString::from("webp")]
                .contains(&ext.to_ascii_lowercase())
        }
        None => false
    }
}

/// This function checks if a given file exists
///
/// arguments:
/// path: PathBuf - path to the file to check
///
/// returns:
/// boolean indicating if the file exists or not
fn file_exists(path: PathBuf) -> bool {
    match path.try_exists() {
        Ok(bool) => bool,
        Err(e) => {
            error!("{}", e.to_string());
            false
        }
    }
}