use std::collections::VecDeque;
use std::error::Error;
use std::{env, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: VecDeque<String>) -> Result<Config, &'static str> {
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        if args.len() < 3 {
            Err("Not enough arguments")
        } else {
            Ok(Config {
                file_path: args
                    .pop_back()
                    .expect("Should have been able to get a path"),
                query: args
                    .pop_back()
                    .expect("Should have been able to get a query"),
                ignore_case,
            })
        }
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut resultat = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            resultat.push(line);
        }
    }
    resultat
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn multi_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick duct three.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick duct three."],
            search(query, contents)
        );
    }

    #[test]
    fn empty_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, hola.
Pick plop three.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
