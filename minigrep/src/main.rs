//! Troy Giorshev  
//! Started: 2019-11-01  
//! Finished:  
//! From <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>  
//! I haven't gone through Ch 14 yet, so my documentation will be ~~a little~~ rough.  
//! TODO: Find a (the?) Rust style guide.  PEP 8 equivalent?

use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect(); // Iterators as usual

    // println!("{:?}", args); // As usual, [0] is the path to the binary itself

    let config = Config::new(&args).unwrap_or_else(|e| {
        eprintln!("Problem parsing arguments: {}", e);
        process::exit(1);
    });

    // No unwrapping needed here
    if let Err(e) = minigrep::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
