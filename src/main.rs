use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
	let config = match Config::build(&args) {
		Ok(t) => t,
		Err(e) => {
			println!("Error when parsing arguments. Error: {}", e);
			process::exit(1);
		}
	};

    dbg!(&config, &config.query);
    let file = match fs::read_to_string(&config.path) {
        Ok(some) => Some(some),
        Err(e) => panic!("Error when opening file. {}", e),
    };
    println!("{:#?}", file);
}

#[derive(Debug)]
struct Config {
    query: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments") //&format!("Expected two arguments. Got {}", args.len() -1 ));
        }
        return Ok(Self {
            query: args[1].clone(),
            path: args[2].clone(),
        });
    }
}
