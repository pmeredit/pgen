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
pub mod sanity;
pub mod mongo_functions;

#[test]
fn parse_topexpr_test() {
    

    assert_eq!(&format!("{:?}", parser::parse_TopExpr("22 * 44 + 66").unwrap()),
                               "((22 * 44) + 66)");
    assert_eq!(&format!("{:?}", parser::parse_TopExpr("3 + (let x=1 in x) + $y + (if foo(x,y,z) then false else x)").unwrap()),
indoc!(
"(((3 + 
let
  \"x\"=1
in
  ID(\"x\")
) + Col(\"$y\")) + 
if App(\"foo\"([ID(\"x\"), ID(\"y\"), ID(\"z\")]))
then false
else ID(\"x\")
)
"));

    assert_eq!(&format!("{:?}", parser::parse_TopExpr("let x = 2 in switch{ x+4 => x, true => y default: '42'}").unwrap()),
indoc!(
"\n
let
  \"x\"=2
in
  
switch{
  (ID(\"x\") + 4) => ID(\"x\")
  true => ID(\"y\")
  default: Str(\"42\")
}

"));

    assert_eq!(&format!("{:?}", parser::parse_TopExpr("foo($x==3 or $y and $z <= null, let z=3,y=42 in z+foo(41+y))").unwrap()),
indoc!(
"App(\"foo\"([((Col(\"$x\") == 3) or (Col(\"$y\") and (Col(\"$z\") <= <null>))), 
let
  \"z\"=3
  \"y\"=42
in
  (ID(\"z\") + App(\"foo\"([(41 + ID(\"y\"))])))
]))"
    ));

    assert_eq!(&format!("{:?}", parser::parse_TopExpr("{'zed' : [1,let x = 3 in x+42,3+x], 'hello' : 'world'}").unwrap()),
indoc!(
"
{\"zed\": [1, 
let
  \"x\"=3
in
  (ID(\"x\") + 42)
, (3 + ID(\"x\"))],\"hello\": Str(\"world\"),}
"
    ));
}

#[test]
fn parse_pipeline_test() {
    assert_eq!(&format!("{:?}", parser::parse_Pipeline("project: {'x':true, 'y': 3 + (let z = $y in foo(z,3.4,-23,[1,2,3,4])) + 23}; match: {'hello' : 42}; sort: {'world' : -1}").unwrap()),
indoc!(
"
[(\"project\", {\"x\": true,\"y\": ((3 + 
let
  \"z\"=Col(\"$y\")
in
  App(\"foo\"([ID(\"z\"), 3.4, -23, [1, 2, 3, 4]]))
) + 23),}), (\"match\", {\"hello\": 42,}), (\"sort\", {\"world\": -1,})]
"
));

}

pub fn process_file(file_name: &str) -> Option<Vec<(String, Box<ast::Expr>)>> {
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

#[cfg(not(test))]
fn main() {
        use std::env;
        use std::fs::File;
        use std::io::prelude::*;

        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            println!("Please pass a file to parse")
        }
        else {
            let ast = process_file(&args[1]);
            if let Some(ast) = ast {
                println!("Ast:\n{:?}", ast);
            }
        }
}
