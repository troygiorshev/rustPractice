/// Troy Giorshev  
/// Started: 2019-11-01  
/// Finished:  
/// From <https://doc.rust-lang.org/book/ch12-00-an-io-project.html>  
/// I haven't gone through Ch 14 yet, so my documentation will be ~~a little~~ rough.
/// TODO: Find a (the?) Rust style guide.  PEP 8 equivalent?

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();  // Iterators as usual
    // println!("{:?}", args); // As usual, [0] is the path to the binary itself

    let pattern = &args[1];
    let filename = &args[2];

    println!("Search Pattern: {}", pattern);
    println!("In file: {}", filename);

    let contents = fs::read_to_string(filename)
        .expect(&format!("Reading file `{}` failed", filename));

    println!("\nText:\n\n{}", contents);
}
