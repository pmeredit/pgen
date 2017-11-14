extern crate regex;
extern crate lalrpop_util;

//This only applies to one import.  We do not want to warn
//on indoc just every time we aren't running tests
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;

#[macro_use]
extern crate lazy_static;

#[allow(unused_macros)]
#[macro_use]
pub mod util;
pub mod ast;
pub mod parser;
pub mod normalize;
pub mod mongo_config;
mod tests;

pub fn process_file(file_name: &str) -> Option<ast::Pipeline> {
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

    let maybe_pipe = parser::parse_Pipeline(&contents);
    match maybe_pipe {
        Ok(p)  => {
            return Some(p)
        },  
        Err(e) => {
            println!("{:?}", e); 
            return None
        }   
    }   
}
