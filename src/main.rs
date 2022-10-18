use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments. Got error: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.path);
    if let Err(e) = run(config) {
        eprintln!("Internal error, {e}");
        process::exit(1);
    };
}
