use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = args.next().ok_or("Missing query string")?;
        let filename = args.next().ok_or("Missing filename")?;
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, filename, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let matches = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in matches {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut matches = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }

    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
I used too much duct tape in my home
Someone please save me I can't get out";

        assert_eq!(vec!["I used too much duct tape in my home"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
I am so miffed that the Rust handbook ended up using the same word I did as last time.
I thought I was being SO PRODUCTIVE by saying
the phrase
'duct tape'.";
        
        assert_eq!(
            vec!["'duct tape'."],
            search(query, contents),
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "BeaR";
        let contents = "\
I won't even try anymore.
It's just too much to bear.
Sorry, Rust-Bear.";
        
        assert_eq!(
            vec!["It's just too much to bear.", "Sorry, Rust-Bear."],
            search_case_insensitive(query, contents),
        );
    }
}