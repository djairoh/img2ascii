//! This module contains functions related to output.
use std::fs::File;
use std::io::{stdout, Write};
use std::path::PathBuf;
use std::process::exit;
use crossterm::execute;
use crossterm::style::{Color, SetForegroundColor, SetBackgroundColor, Print, ResetColor};
use log::error;
use crate::model_rgb_ascii::Ascii;

/// This function prints ASCII art to stdout.
///
/// arguments:
/// art: Vec<Vec<Ascii>> - vector of vectors representing the art
/// in_colour: bool - whether to print in colour or as hard white
/// grayscale: bool - whether to print grayscale
pub fn print_terminal(art: Vec<Vec<Ascii>>, in_colour: bool, grayscale: bool) {
    for line in art {
        for ascii in line {
            if in_colour {
                let _ = execute!(stdout(),
                    SetForegroundColor(Color::Rgb {r: ascii.rgb[0], g: ascii.rgb[1], b: ascii.rgb[2]}),
                    Print(ascii.char),
                    ResetColor);
            } else if grayscale {
                let _ = execute!(stdout(),
                    SetForegroundColor(Color::Rgb {r: ascii.col_depth, g: ascii.col_depth, b: ascii.col_depth}),
                    Print(ascii.char),
                    ResetColor);
            } else {
                print!("{}", ascii.char);
            }
        }
        println!();
    }
}

/// This function prints ASCII art to the background of stdout.
///
/// arguments:
/// art: Vec<Vec<Ascii>> - vector of vectors representing the art
/// in_colour: bool - whether to print in colour
/// grayscale: bool - whether to print grayscale
pub fn print_terminal_background(art: Vec<Vec<Ascii>>, in_colour: bool, grayscale: bool) {
    for line in art {
        for ascii in line {
            if in_colour {
                let _ = execute!(stdout(),
                    SetBackgroundColor(Color::Rgb {r: ascii.rgb[0], g: ascii.rgb[1], b: ascii.rgb[2]}),
                    SetForegroundColor(Color::Rgb {r: 255-ascii.rgb[0], g: 255-ascii.rgb[1], b: 255-ascii.rgb[2]}),
                    Print(ascii.char),
                    ResetColor);
            } else if grayscale {
                let _ = execute!(stdout(),
                    SetBackgroundColor(Color::Rgb {r: ascii.col_depth, g: ascii.col_depth, b: ascii.col_depth}),
                    SetForegroundColor(Color::Rgb {r: 255-ascii.col_depth, g: 255-ascii.col_depth, b: 255-ascii.col_depth}),
                    Print(ascii.char),
                    ResetColor);
            } else {
                error!("This should be unreachable!");
                exit(1);
            }
        }
        println!();
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