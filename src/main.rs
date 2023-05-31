use std::env;
use std::process;

use minigrep::Config;

// Continue here: https://doc.rust-lang.org/book/ch12-06-writing-to-stderr-instead-of-stdout.html

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Unable to parse arguments: {err}");
        process::exit(1)
    });

    if let Err(msg) = minigrep::run(&config) {
        println!("Encountered error: {msg}");
        process::exit(1)
    };
}
