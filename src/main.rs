extern crate pgen;
extern crate serde_json;
extern crate clap;

#[cfg(not(test))]
fn main() {
   use clap::{Arg, App};

   let matches = App::new("Mongo DB Aggregation Pipeline Generator")
       .version("0.3")
       .author("Patrick Meredith <pmeredit@gmail.com>")
       .about("Compiles an easy-to-use ML-like langauge to Mongo DB Aggregation Pipeline Json")
       .arg(Arg::with_name("pretty")
            .long("pretty")
            .short("p")
            .help("Sets pretty print on, off by default")
            )
       .arg(Arg::with_name("gobson")
            .long("gobson")
            .short("gb")
            .help("Output pipeline in go bson")
            .conflicts_with("pretty")
            )
       .arg(Arg::with_name("FILE")
            .help("Sets the input file to use")
            .required(true)
            .index(1)
            )
       .get_matches();


   let file = matches.value_of("FILE").unwrap();
   let pretty = matches.is_present("pretty");
   let gobson = matches.is_present("gobson");
   
   match pgen::run_all(file) {
       Ok(json) => {
           if gobson {
               println!("{}", pgen::codegen::to_go_bson(json));
           } else if pretty {
               println!("{}", serde_json::to_string_pretty(&json).unwrap());
           } else {
               println!("{}", serde_json::to_string(&json).unwrap());
           }
       }
       Err(e) => println!("{}", e)
   }
}
