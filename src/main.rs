use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, path) = parse_args(&args);
    dbg!(query, path);
}

fn parse_args(args: &[String]) -> (&str, &str) {
    return (&args[1], &args[2])
}