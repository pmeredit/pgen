extern crate pgen;
extern crate serde_json;

#[cfg(not(test))]
fn main() {
        use std::env;
        use pgen::normalize::Normalize;
        use pgen::codegen::*;

        let args: Vec<_> = env::args().collect();
        if args.len() < 2 {
            println!("Please pass a file to parse")
        }
        else {
            let pipeline = pgen::process_file(&args[1]);
            if let Some(pipeline) = pipeline {
                println!("Pipeline:\n{:?}", pipeline);

                println!("************");
                let normal = pipeline.normalize().unwrap();
                println!("Normalized: \n{:?}", normal);
                println!("************");
                let json: Option<Box<JsonType>> = normal.convert();
                println!("Json: \n{}", serde_json::to_string_pretty(&json).unwrap());
                println!("************");
                println!("Json: \n{}", serde_json::to_string(&json).unwrap());
            }
        }
}
