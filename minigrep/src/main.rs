use minigrep::Config;
use std::collections::VecDeque;
use std::{env, process};

fn main() {
    let args: VecDeque<String> = env::args().collect();

    let config = Config::build(args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
