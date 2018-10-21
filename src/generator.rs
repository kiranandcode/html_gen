

// The generator module follows the standard pattern.

use std::collections::HashMap;
use regex::{Regex, Captures};
use std::str::FromStr;
#[derive(Clone,Debug,PartialEq)]
pub enum GeneratorErrorCoreStrategy {
   Fail,
   Ignore,
   Fixed(String)
}
pub enum GeneratorErrorStrategy {
   Base(GeneratorErrorCoreStrategy),
   Default(HashMap<String,String>, GeneratorErrorCoreStrategy)
}
#[derive(Debug)]
pub enum GeneratorError {
  UndefinedParameter
}
impl FromStr for GeneratorErrorCoreStrategy {
    type Err = ();
    fn from_str(src: &str) -> Result<GeneratorErrorCoreStrategy, ()> {
        return match src {
            "fail" => Ok(GeneratorErrorCoreStrategy::Fail),
            "ignore" => Ok(GeneratorErrorCoreStrategy::Ignore),
            x => {
                 if let Some(ind) = src.find("=") {
                    if ind + 1 < src.len() {
                        let (txt, rem) = src.split_at(ind+1);
                        if txt == "fixed=" {
                            Ok(GeneratorErrorCoreStrategy::Fixed(rem.to_string()))
                        } else {
                            Err(())
                        }
                    } else {
                        Err(())
                    }
                 } else {
                   Err(())
                 }
            },
        };
    }
}

pub fn generate_output(input: String, mapping: HashMap<String, String>, fail_response: &GeneratorErrorStrategy) -> Result<String, GeneratorError> {
 let parameter_regex = Regex::new("\\{([^¬:{}\\\\]*)\\}").unwrap();
 let mut lookup_failed = false;
 let new_string = parameter_regex.replace_all(&input, |caps: &Captures| {
    if let Some(value) = mapping.get(&caps[1]) {
       value
    } else {
       match &fail_response {
           GeneratorErrorStrategy::Base(strategy) => {
               match strategy {
                 GeneratorErrorCoreStrategy::Fail => {
                     lookup_failed = true;
                     ""
                 }
                 GeneratorErrorCoreStrategy::Ignore => {
                     &caps[0]
                 },
                 GeneratorErrorCoreStrategy::Fixed(text) => {
                     text
                 }
               }
           }
           GeneratorErrorStrategy::Default(mapping, strategy) => {
               if let Some(value) = mapping.get(&caps[1]) {
                  value
               } else {
                  match strategy {
                    GeneratorErrorCoreStrategy::Fail => {
                        lookup_failed = true;
                        ""
                    }
                    GeneratorErrorCoreStrategy::Ignore => {
                        &caps[0]
                    },
                    GeneratorErrorCoreStrategy::Fixed(text) => {
                        text
                    }
                  }  
               }
           }
       }  
    }
 });
 if lookup_failed {
    return Err(GeneratorError::UndefinedParameter);
 }
 Ok(new_string.to_string())
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn from_st_works() {
     assert_eq!(GeneratorErrorCoreStrategy::from_str("ignore"), Ok(GeneratorErrorCoreStrategy::Ignore));
     assert_eq!(GeneratorErrorCoreStrategy::from_str("fail"), Ok(GeneratorErrorCoreStrategy::Fail));
     assert_eq!(GeneratorErrorCoreStrategy::from_str("fixed=missing"), Ok(GeneratorErrorCoreStrategy::Fixed("missing".to_string())));
   }
}
