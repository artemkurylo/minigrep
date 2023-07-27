use std::error::Error;
use std::{env, fs};

pub mod minigrip_search;

/// Config struct <br>
/// Keeps the arguments passed from command line
pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item=String>
    ) -> Result<Config, &'static str> {
        // not used
        let _program_name = args.next();

        let query = match args.next() {
            None => return Err("Didn't get a query string"),
            Some(query) => query,
        };

        let path = match args.next() {
            None => return Err("Didn't get a path string"),
            Some(path) => path,
        };

        let ignore_case = match args.next() {
            Some(param) => Self::parse_bool_value(&param),
            None => false,
        };

        Ok(Self { query, path, ignore_case })
    }

    fn parse_bool_value(argument: &str) -> bool {
        argument
            .parse()
            .unwrap_or_else(|_skip| env::var("IGNORE_CASE").is_ok())
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.path)?;

    let lines;
    if config.ignore_case {
        lines = minigrip_search::search(&config.query, &file_content)
    } else {
        lines = minigrip_search::search_case_insensitive(&config.query, &file_content)
    }

    for line in lines {
        println!("{}", line)
    }

    Ok(())
}