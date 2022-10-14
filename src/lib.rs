use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments"); //&format!("Expected two arguments. Got {}", args.len() -1 ));
        }
        return Ok(Self {
            query: args[1].clone(),
            path: args[2].clone(),
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    for line in search(&config.query, &contents) {
        print!("{line}\n");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
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
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
