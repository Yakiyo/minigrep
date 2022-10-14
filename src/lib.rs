use std::{fs, error::Error};


#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments") //&format!("Expected two arguments. Got {}", args.len() -1 ));
        }
        return Ok(Self {
            query: args[1].clone(),
            path: args[2].clone(),
        });
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.path)?;

    println!("With text:\n{contents}");

    Ok(())
}

