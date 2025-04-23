use std::collections::VecDeque;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: VecDeque<String>) -> Result<Config, &'static str> {
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
            })
        }
    }

    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
