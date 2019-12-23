//! Troy Giorshev  
//! Started: 2019-11-01  
//! Finished:  
//! From <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>  
//! I haven't gone through Ch 14 yet, so my documentation will be ~~a little~~ rough.  
//! TODO: Find a (the?) Rust style guide.  PEP 8 equivalent?

use std::env;
use std::fs;
use std::process;

struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // I'll learn a better way than clone() later
        let pattern = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { pattern, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Iterators as usual

    // println!("{:?}", args); // As usual, [0] is the path to the binary itself

    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {}", e);
        process::exit(1);
    });
    // More on closures next chapter

    println!("Search Pattern: {}", config.pattern);
    println!("In file: {}", config.filename);

    let contents = fs::read_to_string(&config.filename)
        .expect(&format!("Reading file `{}` failed", config.filename));

    println!("\nText:\n\n{}", contents);
}
