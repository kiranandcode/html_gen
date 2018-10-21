// Crates
// The crates we'll be using are as follows:
// - *ArgParse* - This is a crate that I used a while back when making another command line application. It provides a very nice rustic interface over a library which produces command line interfaces compliant with most unix/linux standards.
// an old fashioned regex.

extern crate argparse;



// - *Regex* - We'll be taking advantage of this regex crate to make the parsing phase a little easier; while the stdlib provides some pretty useful string matching utilities, they don't quite match up to

extern crate regex;

// Standard Library Imports
// We'll be using the path utilities provided by the standard library to help us navigate the filesystem in a cross platform way.

use std::env;
use std::path;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::io::Write;

// Module structure
// We'll be splitting up our codebase as follows:


mod crawler;
mod parser;
mod generator;

// Command Line Interface
// Clearly this project is going to be a command line application, as the static generator will need to parse a document and construct the components.

// Using argparse - as imported in the preamble, we'll design a sweet and sexy interface to access our application. The main actions we'll allow a user to perform using this application will be as follows:
// - *specify output folder* - by default the output of the compiled files are placed in ~./bin/~ dir, which is made if it does not exist.
// - *specify template folder* - within a non-templated file, when a template reference is used, by default the application searches the 
//  ~./template/~ dir to resolve these references.
// - *specify input folder* - by default the program searches ~./src/~ for the source files to be compiled


fn main() {
 // let mut output_path = String::from("bin");
 // let mut template_path = String::from("template");
 // let mut input_path = String::from("src");
 let mut output_path = String::from("");
 let mut template_path = String::from("");
 let mut input_path = String::from("");
 let mut base_dir : Option<String> = None;
 // [[file:~/Documents/html_gen/html_gen.org::high%20level%20error%20strategy][high level error strategy]]
 let mut opt_strat = generator::GeneratorErrorCoreStrategy::Fail;
 // high level error strategy ends here
 // [[file:~/Documents/html_gen/html_gen.org::high%20level%20error%20strategy][high level error strategy]]
 let mut def_strat = None;
 // high level error strategy ends here
 let mut help_string : Vec<u8> = Vec::new();
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
 
     ap.refer(&mut base_dir)
     .add_argument("BASEDIR", argparse::StoreOption, "the project directory - if not specified, then --input, --template and --output flags must be specified.");
 
     // [[file:~/Documents/html_gen/html_gen.org::high%20level%20error%20args][high level error args]]
     ap.refer(&mut opt_strat)
       .add_option(&["-e", "--error"],
                   argparse::Store,
                   "Fail on the first undefined parameter");
     // high level error args ends here
     // [[file:~/Documents/html_gen/html_gen.org::high%20level%20error%20args][high level error args]]
     ap.refer(&mut def_strat)
       .add_option(&["-d", "--default"],
                   argparse::StoreOption,
                   "Additional mapping for storing default values");
     // high level error args ends here
 
     ap.print_help("htmlgen", &mut help_string);
 
     ap.parse_args_or_exit();
 }
 let help_string = unsafe { String::from_utf8_unchecked(help_string) };
 let mut output_path = if output_path.is_empty() { None } else { Some(output_path) };
 let mut template_path = if template_path.is_empty() { None } else { Some(template_path) };
 let mut input_path = if input_path.is_empty() { None } else { Some(input_path) };
 if base_dir.is_none() && (output_path.is_none() || template_path.is_none() || input_path.is_none()) {
    println!("{}", help_string);
    ::std::process::exit(-1);
 }
 let (output_path, template_path, input_path) = if let Some(bd) = base_dir {
     let bd = Path::new(&bd);
     let error_string = format!("{:?} is not a valid path", bd);
     let alt_output_path = bd.join(Path::new(&"bin")).to_str().expect(&error_string).to_owned();
     let alt_template_path = bd.join(Path::new(&"template")).to_str().expect(&error_string).to_owned();
     let alt_input_path = bd.join(Path::new(&"src")).to_str().expect(&error_string).to_owned();
 
     let output_path = output_path.unwrap_or_else(|| alt_output_path );
     let template_path = template_path.unwrap_or_else(|| alt_template_path );
     let input_path = input_path.unwrap_or_else(|| alt_input_path );
 
     (output_path, template_path, input_path)
 } else {
     (output_path.unwrap(), template_path.unwrap(), input_path.unwrap())
 };
 let output_directory = Path::new(&output_path);
 let input_directory = Path::new(&input_path);
 let template_directory = Path::new(&template_path);
 // [[file:~/Documents/html_gen/html_gen.org::high%20level%20error%20update][high level error update]]
 let def_strat = def_strat.or_else(|| env::var("GOP_HTML_DEFAULTS").ok());
 // high level error update ends here
 // [[file:~/Documents/html_gen/html_gen.org::high%20level%20error%20update][high level error update]]
 let err_strat = match def_strat {
    None => 
       generator::GeneratorErrorStrategy::Base(opt_strat),
    Some(path) => {
       let mapping = { 
           let def_path = Path::new(&path);
           if let Ok(mut file) = File::open(&def_path) {
              let mut def_source = String::new();
              if let Ok(_count) = file.read_to_string(&mut def_source) {
                  parser::parse_source_string(&def_source).ok()
              } else {
                  None
              }
           } else {
               None
           } 
       };
       match mapping {
         Some((name, map)) => 
             generator::GeneratorErrorStrategy::Default(map, opt_strat), 
         None => {
             eprintln!("Encountered error while reading default mapping at {:?}.", path);
             generator::GeneratorErrorStrategy::Base(opt_strat)
         }
       }
    }
 };
 // high level error update ends here
 println!("{:?}", crawler::crawl_directories(&output_directory, &input_directory, &template_directory, &err_strat));
}
