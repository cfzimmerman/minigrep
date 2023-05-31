use std::env;
use std::error::Error;
use std::fs;
use std::ops::Fn;

pub enum SearchMode {
    CaseSensitive,
    CaseInsensitive,
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub mode: SearchMode,
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&config.file_path)?;
    let results = search(&config.query, &contents, config.get_filter());
    for line in results.iter() {
        println!("{line}");
    }
    if results.len() == 0 {
        println!("No results found");
    }
    Ok(())
}

pub fn search<'a>(
    query: &str,
    contents: &'a str,
    fltr: impl Fn(&str, &str) -> bool,
) -> Vec<&'a str> {
    contents.lines().filter(|line| fltr(query, line)).collect()
}

/*
*
* Takeaways for myself (deviating from the tutorial)
* - Passing a function seemed like a good idea, but it forced me into
*   a very inefficient implementation of case_insensitive. That's
*   something to think ahead about.
* - Struct methods are hard to test outside the context of an instantiated struct.
*   Testing case insensitive is probably only viable if I CLI parsing logic and
*   env variable logic to helper functions.
*
*/

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing arguments: (example: -- keyword file.txt)");
        }
        let mode = match env::var("IGNORE_CASE") {
            Err(_) => SearchMode::CaseSensitive,
            Ok(_) => SearchMode::CaseInsensitive,
        };
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            mode,
        })
    }

    pub fn get_filter(&self) -> impl Fn(&str, &str) -> bool {
        match self.mode {
            SearchMode::CaseSensitive => |query: &str, line: &str| line.contains(query),
            SearchMode::CaseInsensitive => {
                |query: &str, line: &str| line.to_lowercase().contains(&query.to_lowercase())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
