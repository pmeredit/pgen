extern crate pgen;

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
            let pipeline = pgen::process_file(&args[1]);
            if let Some(pipeline) = pipeline {
                println!("Pipeline:\n{:?}", pipeline);
            }
        }
}
