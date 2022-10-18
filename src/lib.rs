use std::{env, error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }
        let mut case = false;
        if args.len() > 3 {
            if args[3].to_lowercase() == "true" { case = true; }
        } else if let Ok(t) = env::var("IGNORE_CASE") {
            if t.to_lowercase() == "true" {
                case = true;
            }
        }
        return Ok(Self {
            query: args[1].clone(),
            path: args[2].clone(),
            case_sensitive: case,
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.path)?;
	let val = search(&config.query, &contents, &config.case_sensitive);
	if val.is_empty() {
		println!("No results matching query: {} found in file {}", &config.query, &config.path);
	}
    for line in search(&config.query, &contents, &config.case_sensitive) {
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
