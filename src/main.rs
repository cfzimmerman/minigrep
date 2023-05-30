use std::env;
use std::fs;

// Restart here: Returning a result instead of calling panic
// https://doc.rust-lang.org/book/ch12-03-improving-error-handling-and-modularity.html#returning-a-result-instead-of-calling-panic:~:text=Returning%20a%20Result%20Instead%20of%20Calling%20panic!

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args);

    let contents: String = fs::read_to_string(&config.file_path).expect("Unable to read file");

    println!(
        "read {} from {}: {}",
        &config.query, &config.file_path, &contents
    );
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("missing arguments: (ex) -- keyword file.txt");
        }
        Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        }
    }
}
