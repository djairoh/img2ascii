//! Driver module.
#![feature(file_create_new)]

use clap::Parser;
use crate::ascii_manipulation::*;
use crate::cli::Cli;
use crate::output::*;
use crate::image_manipulation::*;
use crate::model_rgb_ascii::Ascii;

mod cli;
mod image_manipulation;
mod ascii_manipulation;
mod output;
mod model_rgb_ascii;

//todo: general
/* Readme
 */


/// This is the main function.
///
/// It drives the CLI module, calls the appropriate convert functions and directs output.
fn main() {
    std::env::set_var("RUST_LOG", "info");

    //parse flags
    let cli: Cli = Cli::parse();
    cli.init();
    cli.validate();

    //preprocess image
    let mut img = open_image(cli.image);
    img = resize_image(img, cli.full, cli.braille, cli.width);

    //converting to ASCII
    let out: Vec<Vec<Ascii>>;
    if cli.braille {
        out = to_braille_ascii(img, cli.threshold);
    } else if cli.complex {
        out = to_complex_ascii(img);
    } else if let Some(map) = cli.map {
        out = to_custom_ascii(map, img);
    } else {
        out = to_simple_ascii(img);
    }

    //output
    if let Some(output) = cli.output {
        print_file(out, output);
    } else {
        print_terminal(out, cli.colour);
    }
}
