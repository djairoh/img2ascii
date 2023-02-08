//! This module contains functions related to output.
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use log::error;
use termion::{color, style};
use crate::model_rgb_ascii::Ascii;

/// This function prints ASCII art to stdout.
///
/// arguments:
/// art: Vec<Vec<Ascii>> - vector of vectors representing the art
/// in_colour: bool - whether to print in colour or as hard white
pub fn print_terminal(art: Vec<Vec<Ascii>>, in_colour: bool) {
    for line in art {
        for ascii in line {
            if in_colour {
                print!("{}{}", color::Fg(color::Rgb(ascii.rgb[0], ascii.rgb[1], ascii.rgb[2])), ascii.char);
            } else {
                print!("{}", ascii.char);
            }
        }
        println!("{}", style::Reset);
    }
}

/// This function prints ASCII art to a file.
///
/// arguments:
/// art: Vec<Vec<Ascii>> - vector of vectors representing the art
/// out: PathBuf - the path of the out file
///
/// returns:
/// () if everything goes smoothly, Err otherwise
fn _print_file(art: Vec<Vec<Ascii>>, out: PathBuf) -> std::io::Result<()> {
    let mut file = File::create_new(out)?;
    for line in art {
        for ascii in line {
            write!(file, "{}", ascii.char)?;
        }
        writeln!(file)?;
    }
    Ok(())
}

/// This is a wrapper function for _print_file(). It handles any errors that _print_file() throws up.
///
/// arguments:
/// art: Vec<Vec<Ascii>> - vector of vectors representing the art
/// out: PathBuf - the path of the out file
pub fn print_file(art: Vec<Vec<Ascii>>, out: PathBuf) {
    if let Err(e) = _print_file(art, out) {
        error!("Failed to write to file: {}", e.to_string());
        exit(1);
    }
}