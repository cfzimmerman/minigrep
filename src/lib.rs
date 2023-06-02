use std::{env, error::Error, fs, ops::Fn};

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
    let results = search(&config.query, &contents, Config::get_filter(&config.mode));
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

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = args.next().ok_or("Missing query string")?;
        let file_path = args.next().ok_or("Missing file path")?;
        let mode = match env::var("IGNORE_CASE") {
            Err(_) => SearchMode::CaseSensitive,
            Ok(_) => SearchMode::CaseInsensitive,
        };
        Ok(Config {
            query,
            file_path,
            mode,
        })
    }

    pub fn get_filter(mode: &SearchMode) -> impl Fn(&str, &str) -> bool {
        match mode {
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
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.
Duct tape";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(
                query,
                contents,
                Config::get_filter(&SearchMode::CaseSensitive)
            )
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "ruST";
        let contents = "\
Rust:
save, fast, productive. 
Pick three.
Trust me";
        assert_eq!(
            vec!["Rust:", "Trust me"],
            search(
                query,
                contents,
                Config::get_filter(&SearchMode::CaseInsensitive)
            )
        )
    }
}
