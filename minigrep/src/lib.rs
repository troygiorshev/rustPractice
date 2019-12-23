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

        let pattern = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { pattern, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(&config.pattern, &contents) {
        println!("{}", line)
    }

    Ok(()) // This says "we're using this fn for its side effects only"
}

fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    
    for line in contents.lines() {
        if line.contains(pattern){
            ret.push(line)
        }
    }

    ret
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let pattern = "duct";
        // This is an interesting way of doing long multi-line strings
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }
}
