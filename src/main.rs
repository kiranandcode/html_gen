// Crates
// The crates we'll be using are as follows:
// - *ArgParse* - This is a crate that I used a while back when making another command line application. It provides a very nice rustic interface over a library which produces command line interfaces compliant with most unix/linux standards.


extern crate argparse;

// Standard Library Imports
// We'll be using the path utilities provided by the standard library to help us navigate the filesystem in a cross platform way.

use std::path;

// Module structure
// We'll be splitting up our codebase as follows:


mod parser;
mod crawler;

// Command Line Interface
// Clearly this project is going to be a command line application, as the static generator will need to parse a document and construct the components.

// Using argparse - as imported in the preamble, we'll design a sweet
// and sexy interface to access our application.
// The main actions we'll allow a user to perform using this application
// will be as follows:
// - *specify output folder* - by default the output of the compiled files are placed in ~./bin/~ dir, which is made if it does not exist.
// - *specify template folder* - within a non-templated file, when a template reference is used, by default the application searches the 
//  ~./template/~ dir to resolve these references.
// - *specify input folder* - by default the program searches ~./src/~ for templated files to be 


fn main() {
 let mut output_path = String::from("./bin");
 let mut template_path = String::from("./template");
 let mut input_path = String::from("./src");
 {
     let mut ap = argparse::ArgumentParser::new();
     ap.set_description("Simple templating engine for html documents");
     ap.refer(&mut output_path)
     .add_option(&["-o","--output"], 
                 argparse::Store, 
                 "directory for the output files to be saved");
 
     ap.refer(&mut template_path)
     .add_option(&["-t","--template"], 
                 argparse::Store, 
                 "directory to be searched to find templates");
 
     ap.refer(&mut input_path)
     .add_option(&["-i","--input"], 
                 argparse::Store, "
                 directory in which the source files to be compiled are located");
 
 
     ap.parse_args_or_exit();
 }
}
