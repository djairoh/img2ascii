use clap::Parser;
use crate::ascii_manipulation::*;
use crate::cli::Cli;
use crate::output::*;
use crate::image_manipulation::*;

mod cli;
mod image_manipulation;
mod ascii_manipulation;
mod output;

//todo: general
/* Documentation
 * Readme
 * https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust
 */

fn main() {
    std::env::set_var("RUST_LOG", "info");

    //parse flags
    let cli: Cli = Cli::parse();
    cli.init();
    cli.validate();

    //preprocess image
    let mut img = open_image(cli.image);
    img = resize_image(img, cli.full, cli.width);

    //converting to ASCII
    let out: Vec<String>;
    if cli.braille {
        out = to_braille_ascii(img);
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

    //todo:
    /* function that converts image to braille (if -b)
     * something about printing in colour
     * output to file
     */

}
