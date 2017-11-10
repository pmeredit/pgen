extern crate lalrpop_util;

//This only applies to one import.  We do not want to warn
//on indoc just every time we aren't running tests
#[allow(unused_imports)]
#[macro_use]
extern crate indoc;

pub mod ast;
pub mod parser;

#[test]
fn parser() {
    
    assert_eq!(&format!("{:?}", parser::parse_TopExpr("22 * 44 + 66").unwrap()),
                               "((22 * 44) + 66)");
    assert_eq!(&format!("{:?}", parser::parse_TopExpr("3 + (let x=1 in x) + $y + (if foo(x,y,z) then false else x)").unwrap()),
indoc!(
"(((3 + 
let
  ID(\"x\")=1
in
  ID(\"x\")
) + Col(\"$y\")) + 
if App(\"foo\"([ID(\"x\"), ID(\"y\"), ID(\"z\")]))
then false
else ID(\"x\")
)
"));
    assert_eq!(&format!("{:?}", parser::parse_TopExpr("let x = 2 in switch{ x+4 => x, true => y default: 4}").unwrap()),
indoc!(
"\n
let
  ID(\"x\")=2
in
  
switch{
  (ID(\"x\") + 4) => ID(\"x\")
  true => ID(\"y\")
  default: 4
}

"));
}

#[cfg(not(test))]
fn main() {
        use std::env;
        
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            println!("Please pass an expression to parse")
        }
        else {
            println!("{:?}", parser::parse_TopExpr(&args[1])) 
        }
}
