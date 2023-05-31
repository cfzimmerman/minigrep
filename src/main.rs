use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Unable to parse arguments: {err}");
        process::exit(1)
    });

    if let Err(msg) = minigrep::run(&config) {
        eprintln!("Encountered error: {msg}");
        process::exit(1)
    };
}
