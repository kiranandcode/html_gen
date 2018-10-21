

// The main structure for the crawler is as follows.

use std::fs;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use parser::{parse_source_string,ParseError};
use generator::{generate_output, GeneratorError, GeneratorErrorStrategy};

#[derive(Debug)]
pub enum CrawlError {
  ParseError(ParseError),
  GeneratorError(GeneratorError),
  TemplateNotFound(String),
  InputDirectoryError,
  OutputFileError(String),
  InputFileError(String),
}

pub fn crawl_directories<P,Q,R>(
    output_directory: &P, 
    input_directory: &Q, 
    template_path: &R, 
    err_strat: &GeneratorErrorStrategy
) -> Result<u32,CrawlError> 
 where P : AsRef<Path>,
       Q : AsRef<Path>,
       R : AsRef<Path> {
let mut file_count = 0;
let input_files = input_directory.as_ref()
                  .read_dir()
                  .map_err(|_| 
                        CrawlError::InputDirectoryError
                  )?;
for input_file in input_files {
   let input_file = input_file.map_err(|e| CrawlError::InputFileError(format!("{:?}", e)))?;
   let input_metadata = input_file.metadata().map_err(|e| CrawlError::InputFileError(format!("{:?}", e)))?;
   let input_file_name = input_file.file_name();
   let input_file_path = input_file.path();
   let input_file_extension = input_file_path.extension().and_then(|ext| ext.to_str());
   let input_file_base = input_file_path.file_stem().and_then(|stem| stem.to_str());
   if input_metadata.is_dir() {
       let dir_name = Path::new(&input_file_name);
       let new_output_dir = output_directory
                            .as_ref()
                            .join(&dir_name);
       let new_input_dir = input_directory
                           .as_ref()
                           .join(&dir_name);
       let n_count = crawl_directories(
           &new_output_dir, 
           &new_input_dir, 
           template_path, 
           err_strat
       )?;
       file_count += n_count;
   } else if input_metadata.is_file() && (input_file_extension == Some("gop")) && (input_file_base.is_some()) {
       let input_text = {
          let mut temp = String::new();
          let mut file = File::open(input_file.path()).map_err(|e| CrawlError::InputFileError(format!("{:?}", e)))?;
          file.read_to_string(&mut temp).map_err(|e| CrawlError::InputFileError(format!("{:?}", e)))?;
          temp
       };
       let (template_name, mapping) = parse_source_string(&input_text).map_err(|e| CrawlError::ParseError(e))?;
       let template_path = template_path.as_ref().join(&Path::new(&template_name));
       let template_text = {
          let mut temp = String::new();
          let mut file = File::open(template_path).map_err(|e| CrawlError::TemplateNotFound(format!("{:?}", e)))?;
          file.read_to_string(&mut temp).map_err(|e| CrawlError::TemplateNotFound(format!("{:?}", e)))?;
          temp
       };
       let result = generate_output(
          template_text, 
          mapping, 
          err_strat
       ).map_err(|e| CrawlError::GeneratorError(e))?;
       let input_file_base = input_file_base.unwrap();
       let mut new_file_name = String::from(input_file_base);
       new_file_name.push_str(".html");
       let output_path = 
           output_directory.as_ref().join(&Path::new(&new_file_name));
       fs::write(&output_path, result)
           .map_err(|e| CrawlError::OutputFileError(format!("{:?}", e)))?;
       file_count += 1;
   } else {
      eprintln!("WARN: Encountered a non-template file (or non unicode path) during crawling the input directory {:?}", input_file);
   }
}
Ok(file_count)
}
