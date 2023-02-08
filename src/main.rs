use clap::Parser;
use image::imageops::FilterType;
use crate::ascii_manipulation::to_simple_ascii;
use crate::cli::Cli;
use crate::output::print_terminal;

mod cli;
mod image_manipulation;
mod ascii_manipulation;
mod output;

//todo: general
/* https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust
 */

fn main() {
    std::env::set_var("RUST_LOG", "info");

    //parse flags
    let cli: Cli = Cli::parse();
    cli.init();
    cli.validate();

    let mut img = image_manipulation::open_image(cli.image);
    let w = image_manipulation::get_size(cli.width, img.width());

    //todo: change logic to include -full flag for max width, otherwise use max height?
    let h: u32 = (img.height() as f32 * w as f32 / img.width() as f32 * 0.5) as u32;
    img = img.resize_exact(w as u32, h, FilterType::CatmullRom);

    let out = to_simple_ascii(img);

    print_terminal(out);

    //todo:
    /* function that converts image to braille (if -b)
     * something about printing in colour
     * a function to let the user define a custom map
     */

}
