use std::path::PathBuf;

pub fn print_terminal(ascii: Vec<String>) {
    for line in ascii {
        println!("{}", line);
    }
}

pub fn print_file(ascii: Vec<String>, out: PathBuf) {
    //todo: output
}