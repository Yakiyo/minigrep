use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::build(&args) {
        Ok(t) => t,
        Err(e) => {
            println!("Error when parsing arguments. Error: {}", e);
            process::exit(1);
        }
    };

    println!("Searching for {}", config.query);
    println!("In file {}", config.path);
    if let Err(e) = run(config) {
        println!("Internal error, {e}");
        process::exit(1);
    };
}
