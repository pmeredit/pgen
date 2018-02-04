extern crate regex;
extern crate lalrpop_util;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

//This only applies to one import.  We do not want to warn
//on indoc just every time we aren't running tests
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;

//#[macro_use(bson, doc)]
//extern crate bson;

#[macro_use]
extern crate lazy_static;

extern crate linked_hash_map;

#[allow(unused_macros)]
#[macro_use]
pub mod util;
pub mod ast;
pub mod codegen;
pub mod parser;
pub mod normalize;
pub mod mongo_config;
mod tests;

pub fn process_file(file_name: &str) -> Result<ast::Pipeline, String> {
    use std::fs::File;
    use std::io::prelude::*;
    use regex::Regex;
    
    // Open and read file
    let mut f = File::open(&file_name).expect("File Not Found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Could Not Read File");
    
    // Remove comments
    let single_line_comment_re = Regex::new(r"//.*" ).unwrap();
    let multiline_comment_re = Regex::new(r"/\*.*\*/").unwrap();
    
    let contents = single_line_comment_re.replace_all(&contents, "");
    let contents = multiline_comment_re.replace_all(&contents, "");

    parser::parse_Pipeline(&contents).map_err(|e| format!("{:?}", e))
}
