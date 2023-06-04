use minigrep::Config;
use std::{env, process};

fn main() {
    // Read command line arguments
    let args: Vec<String> = env::args().collect();

    // Gather configuration data
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Run program
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}
