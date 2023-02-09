# img2ascii
This Command-Line tool converts input images into ASCII art.

It can be configured using flags on the command line, a few options of which are:
 * Simple ASCII (using the map ` .:-=+*#%@"`)
 * Complex ASCII (using the map ``` .'`^",:;Il!i><~+_-?][}{1)(|\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$```)
 * [Braille symbols](https://en.wikipedia.org/wiki/Braille_Patterns)
 * Custom character maps (through the `--map` flag)
 * Output in full colour
 * Output in grayscale

# Prerequisites
* [Rust](https://www.rust-lang.org/) (building from source only)

# Installation
Installing is pretty easy,
just grab the appropriate executable from the releases tab,
and you're good to go!

## Building from source
```shell
git clone https://github.com/djairoh/img2ascii
cd ./img-search
cargo build --release
mv ./target/release/
./img2ascii <FILE> [FLAGS]
```

# Running
Running the program is very simple. 
Simply execute `./img2ascii <IMAGE>` and it will convert <IMAGE> to ASCII and print to stdout.

Of course the output can be customized, by using one (or more) flags.
## options
 * -c, --complex: uses the extended character map
 * -C, --colour: Display ASCII art in full 24-bit colour
 * --background: Colour the background instead of the foreground
 * -g, --grayscale: Display ASCII art in grayscale
 * -b, --braille: Use braille characters instead of a character map
 * -d, --debug: Print debugging information
 * -f, --full: use the full width of the terminal, instead of fitting the image to the terminal dimensions
 * --threshold: use a custom threshold when converting to braille (range 0-255, default: 128)
 * -M, --map: use a custom character map
 * -w, --width: force a specific width of the output
 * -o, --output: Save the output to a text file, instead of printing to terminal
 * -h, --help: Print help information (extended info with '--help')

## conflicting flags
The flags `--complex`, `--braille`, `--map` conflict.
When two or more of these are entered in the same command,
the program uses the one with the highest precedence, in the order
`braille > complex > map`.

The flags `--output`, `--colour` and `--grayscale` also conflict. 
When both are entered, the flag `--output` takes precedence
(leading to a simple txt file).

The flags `--full` and `--width` are mutually exclusive.
When both are entered, the flag `-full` will be applied
(`full > width`).

Lastly, the flags `--colour` and `--grayscale` are not mutually applicable.
When both are entered, the flag `--colour` will be applied.

Note also that the `--background` flag requires either `--colour` or `--grayscale` to do anything.

# Documentation
The program has been documented with RustDoc,
which can be compiled and viewed with
`cargo doc --open`.


