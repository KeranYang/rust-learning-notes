use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

// dyn Error means the function can return any type that implements the Error trait.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// search function that returns a vector of string slices, each slice representing a line that contains the query.
// Note: we need the lifetime annotation 'a to indicate that the returned string slices will live at least as long as the contents parameter.
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
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
    fn test_search_case_sensitive_found() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn test_search_case_sensitive_not_found() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, prodUctive.
Pick three.";
        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, prodUctive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, prodUctive."],
            search_case_insensitive(query, contents)
        );
    }
}