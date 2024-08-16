use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query : String,
    pub filename : String,
    pub case_sensitive : bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }
        args.next();
        let query = match args.next() {
            Some(v) => v,
            None => return Err("Didn't get a string"),
        };
        let filename = match args.next() {
            Some(v) => v,
            None => return Err("DIdn't get a filename"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    } 
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }
    // result

    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query.to_lowercase()) {
    //         result.push(line);
    //     }
    // }
    // result

    contents.lines().filter(|line| line.to_lowercase().contains(query)).collect()
}


#[cfg(test)] 
mod tests {
    use super::*;

    #[test] 
    fn case_sensitive() {
        let query = "duct";
        let contents =  "\
Rust:
safe, fast, productive,
Duct tape.";
        assert_eq!(vec!["safe, fast, productive,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents =  "\
Rust:
safe, fast, productive,
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_insensitive(query, contents));
    }
}