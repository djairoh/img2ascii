use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use log::error;
use termion::{color, style};
use crate::model_rgb_ascii::Ascii;

pub fn print_terminal(art: Vec<Vec<Ascii>>, in_colour: bool) {
    for line in art {
        for ascii in line {
            if in_colour {
                print!("{}{}", color::Fg(color::Rgb(ascii.rgb.r, ascii.rgb.g, ascii.rgb.b)), ascii.char);
            } else {
                print!("{}", ascii.char);
            }
        }
        println!("{}", style::Reset);
    }
}

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

pub fn print_file(art: Vec<Vec<Ascii>>, out: PathBuf) {
    if let Err(e) = _print_file(art, out) {
        error!("Failed to write to file: {}", e.to_string());
        exit(1);
    }
}