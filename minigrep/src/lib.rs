use std::error::Error;
use std::fs;

pub struct Config {
    pattern: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // I'll learn a better way than clone() later
        let pattern = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { pattern, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Search Pattern: {}", config.pattern);
    println!("In file: {}", config.filename);

    let contents = fs::read_to_string(&config.filename)?;

    println!("\nText:\n\n{}", contents);

    Ok(()) // This says "we're using this fn for its side effects only"
}
