use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use log::error;
use crate::model_rgb_ascii::Ascii;

//todo: take into consideration the in_colour flag
pub fn print_terminal(art: Vec<Vec<Ascii>>, in_colour: bool) {
    for line in art {
        for ascii in line {
            print!("{}", ascii.char)
        }
        println!();
    }
}

pub fn print_file(art: Vec<Vec<Ascii>>, out: PathBuf) -> std::io::Result<()> {
    let mut file = File::create_new(out)?;
    for line in art {
        for ascii in line {
            write!(file, "{}", ascii.char)?;
        }
        writeln!(file)?;
    }
    Ok(())
}