extern crate lalrpop_util;
#[macro_use]
extern crate indoc;

use std::env;

pub mod ast;
pub mod parser;

#[test]
fn parser() {
    assert_eq!(&format!("{:?}", parser::parse_TopExpr("22 * 44 + 66").unwrap()),
                               "((22 * 44) + 66)");
    assert_eq!(&format!("{:?}", parser::parse_TopExpr("3 + (let x=1 in x) + $y + (if true then false else x)").unwrap()),
indoc!(
"(((3 + 
let
  ID(\"x\")=1
in
  ID(\"x\")
) + Col(\"$y\")) + 
if true
then false
else ID(\"x\")
)
"));
}

#[cfg(not(test))]
fn main() {
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            println!("Please pass an expression to parse")
        }
        else {
            println!("{:?}", parser::parse_TopExpr(&args[1])) 
        }
}
