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
, (3 + ID(\"x\"))]\"hello\": Str(\"world\")}
"
    ));
}

#[cfg(not(test))]
fn main() {
        use std::env;
        
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            println!("Please pass an expression to parse")
        }
        else {
            println!("{:?}", parser::parse_Pipeline(&args[1])) 
        }
}
