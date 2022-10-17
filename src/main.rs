use minigrep::{run, Config};
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
	let config: Config = Config::build(&args).unwrap_or_else(|err| {
		println!("Problem with parsing arguments. Got error: {err}");
		process::exit(1);
	});

    println!("Searching for {}", config.query);
    println!("In file {}", config.path);
    if let Err(e) = run(config) {
        println!("Internal error, {e}");
        process::exit(1);
    };
}
