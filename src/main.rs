// Crates
// The crates we'll be using are as follows:
// - *ArgParse* - This is a crate that I used a while back when making
// another command line application. It provides a very nice rustic
// interface over a library which produces command line interfaces
// compliant with most unix/linux standards.


// [[file:~/Documents/html_gen/html_gen.org::*Crates][Crates:1]]
extern crate argparse;
// Crates:1 ends here

// Standard Library Imports
// We'll be using the path utilities provided by the standard library to help us navigate the filesystem in a cross platform way.

// [[file:~/Documents/html_gen/html_gen.org::*Standard%20Library%20Imports][Standard Library Imports:1]]
use std::path;
// Standard Library Imports:1 ends here

fn main() {

let mut output_path = String::from("./bin");
let mut template_path = String::from("./template");
let mut input_path = String::from("./src");

{
    let mut ap = argparse::ArgumentParser::new();
    ap.set_description("Simple templating engine for html documents");
    ap.refer(&mut output_path)
    .add_option(&["-o","--output"], argparse::Store, "directory for the output files to be saved");

    ap.refer(&mut template_path)
    .add_option(&["-t","--template"], argparse::Store, "directory to be searched to find templates");

    ap.refer(&mut input_path)
    .add_option(&["-i","--input"], argparse::Store, "directory in which the source files to be compiled are located");


    ap.parse_args_or_exit();
}

}
