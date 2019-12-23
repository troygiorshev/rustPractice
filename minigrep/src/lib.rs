use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pattern: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let pattern = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { pattern, filename , case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.pattern, &contents)
    } else {
        search_case_insensitive(&config.pattern, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(()) // This says "we're using this fn for its side effects only"
}

fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let mut ret = Vec::new();
    
    for line in contents.lines() {
        if line.contains(&pattern){
            ret.push(line);
        }
    }

    ret
}

fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut ret = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
            ret.push(line);
        }
    }

    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        // This is an interesting way of doing long multi-line strings
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
I love Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn case_insensitive() {
        let pattern = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(pattern, contents));
    }
}
