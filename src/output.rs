use std::path::PathBuf;
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

pub fn print_file(ascii: Vec<Vec<Ascii>>, out: PathBuf) {
    //todo: this
}