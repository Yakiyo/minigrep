use std::{error::Error, fs, env};

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
		if args.len() > 3 && args[3].to_lowercase() == "true" {
			case = true;
		} else if let Ok(t) = env::var("IGNORE_CASE") {
			if t.to_lowercase() == "true" {
				case = true;
			}
		}
		println!("{}", case);
        return Ok(Self {
            query: args[1].clone(),
            path: args[2].clone(),
			case_sensitive: case,
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
			Rust:\
			safe, fast, productive.\
			Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
