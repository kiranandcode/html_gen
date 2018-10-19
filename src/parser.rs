// Parser Logic
// Before we begin, we'll need the following packages in our parser:

use std::collections::HashMap;
use regex::Regex;
#[derive(Debug)]
pub enum ParseError {
   TemplateNotFound,
   InvalidIdentifier,
   UnterminatedBody
}


// Once again, our core specification for the parser is to extract a set of key value pairs. Our syntax will be of the following form:
// #+begin_src 
// ID := (Sigma/{:, (, )})+
// INTRO := #+template: Sigma+\n
// MAPPING := ID:  ((SIGMA/{¬})|\¬)* ¬
// DOCUMENT := INTRO MAPPING*
// #+end_src
// Our parser will take in a string (the contents of the file), and return either a hashmap of values and a template name, or an error.

fn split_at_regex<'a>(string: &'a str, pat: &Regex) -> (&'a str, &'a str) {
  if let Some(m) = pat.find(string) {
     string.split_at(m.end())
  } else {
     (&"", string)
  }
}
fn split_at_pattern<'a>(string: &'a str, pat: &str) -> (&'a str, &'a str) {
  if let Some(ind) = string.find(pat) {
     string.split_at(ind)
  } else {
     (&"", string)
  }
}

pub fn parse_source_string(source: &str) 
   -> Result<(String, HashMap<String,String>),ParseError> {
let key_regex = Regex::new("^[^¬:{}\\\\]*:").unwrap();
let data_regex = Regex::new("^(\\\\¬|([^¬\\\\]|\\\\[^¬])*)*¬").unwrap();
if !source.trim_left().starts_with("#+template:") {
   return Err(ParseError::TemplateNotFound);
}
let source = source.trim_left().split_at(11).1;
let (raw_template_name, remaining_string) = split_at_pattern(source, "\n");
let template_name = raw_template_name.trim();
if template_name.is_empty() {
   return Err(ParseError::TemplateNotFound);
}
let mut data : HashMap<String, String> = HashMap::new();
let mut completed = false;
let mut source = remaining_string;
let mut data = data;

while !completed {
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   let (raw_key_name, remaining_string) = split_at_regex(source, &key_regex);
   let key_name = raw_key_name.trim();
   source = remaining_string;
   // source pairs loop ends here
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   if key_name.len() == 0 {
     eprintln!("Invalid parse, found empty/malformed ID tag");
     return Err(ParseError::InvalidIdentifier);
   }
   // source pairs loop ends here
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   let mut key_name = key_name.to_string();
   key_name.pop();
   let key_name = key_name.trim();
   // source pairs loop ends here
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   let (raw_data, remaining_string) = split_at_regex(source, &data_regex);
   let src_data = raw_data.trim();
   source = remaining_string;
   // source pairs loop ends here
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   if src_data.len() == 0 {
     eprintln!("Invalid parse, found body with no terminating tag.");
     return Err(ParseError::UnterminatedBody);
   }
   // source pairs loop ends here
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   let mut src_data = src_data.to_string();
   src_data.pop();
   let src_data = src_data.trim();
   // source pairs loop ends here
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   data.insert(key_name.to_string(), src_data.to_string());
   // source pairs loop ends here
   // [[file:~/Documents/html_gen/html_gen.org::source%20pairs%20loop][source pairs loop]]
   if source.trim().is_empty() {
       break;
   }
   // source pairs loop ends here
}
Ok((template_name.to_string(), data))
}

#[cfg(test)]
mod test {
   use super::*;

  #[test]
  fn must_start_with_template_directive() {
     assert!(parse_source_string("temp-justkidding\n id:\n #+template:\n").is_err());
  }
  #[test]
  fn must_provide_template_name() {
      assert!(parse_source_string("#+template: example\n").is_ok());
      assert!(parse_source_string("#+template:\n").is_err());
      assert!(parse_source_string("#+template:    \n").is_err());
      assert!(parse_source_string("#+template:   \n  \n").is_err());
      assert!(parse_source_string("#+template:   \t  \n").is_err());
  }
}
