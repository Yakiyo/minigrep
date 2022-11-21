use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        // Consume the first item since it only returns the directory path
        args.next();
        let mut case = false;
        if let Ok(t) = env::var("IGNORE_CASE") {
            if t.to_lowercase() == "true" {
                case = true;
            }
        }

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // override case with args value **if** it exists, else use env or default
        if let Some(t) = args.next() {
            if t.to_lowercase() == "true" {
                case = true;
            } else {
                case = false;
            }
        };

        Ok(Config {
            query,
            path,
            case_sensitive: case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
    let val = search(&config.query, &contents, &config.case_sensitive);
    if val.is_empty() {
        println!(
            "No results matching query: \"{}\" found in file \"{}\"",
            &config.query, &config.path
        );
    }
    for line in val {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case: &bool) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();

    if case == &(false) {
        for line in contents.lines() {
            if line.contains(query) {
                res.push(line);
            }
        }
    } else {
        let query = query.to_lowercase();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query) {
                res.push(line);
            }
        }
    }
    return res;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn search_test() {
        let query = "duct";
        let case = false;
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, &case)
        );
    }
    #[test]
    fn search_test_insensitive() {
        let query = "DuCt";
        let case = true;
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, &case)
        );
    }
}
