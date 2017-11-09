extern crate lalrpop_util;

use std::env;

pub mod ast;
pub mod parser;

#[test]
fn parser() {
    assert_eq!(&format!("{:?}", parser::parse_Exprs("").unwrap()),
                               "[]");
    assert_eq!(&format!("{:?}", parser::parse_Exprs("22 * 44 + 66").unwrap()),
                               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", parser::parse_Exprs("22 * 44 + 66,").unwrap()),
                               "[((22 * 44) + 66)]");
    assert_eq!(&format!("{:?}", parser::parse_Exprs("22 * 44 + 66, 13*3").unwrap()),
                               "[((22 * 44) + 66), (13 * 3)]");
    assert_eq!(&format!("{:?}", parser::parse_Exprs("22 * 44 + 66, 13*3,").unwrap()),
                               "[((22 * 44) + 66), (13 * 3)]");
}

#[cfg(not(test))]
fn main() {
        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            println!("Please pass an expression to parse")
        }
        else {
            println!("{:?}", parser::parse_Exprs(&args[1])) 
        }
}
