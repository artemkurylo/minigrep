use std::error::Error;
use std::{env, fs};

pub mod minigrip_search;

pub struct Config {
    pub query: String,
    pub path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }

        let query = args[1].clone();
        let path = args[2].clone();
        let ignore_case = Self::parse_bool_value(&args[3]);

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