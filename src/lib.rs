use std::error::Error;
use std::{env, fs};

const IGNORE_CASE_ENV_VAR: &str = "IGNORE_CASE";

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
            .unwrap_or(
                env::var(IGNORE_CASE_ENV_VAR).is_ok()
            )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = fs::read_to_string(config.path)?;

    let lines;
    if config.ignore_case {
        lines = search(&config.query, &file_content)
    } else {
        lines = search_case_insensitive(&config.query, &file_content)
    }

    for line in lines {
        println!("{}", line)
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut vec = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            vec.push(line.trim());
        }
    }

    vec
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();

    let mut vec = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(query) {
            vec.push(line.trim());
        }
    }

    vec
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "productive";
        let content = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let right = search(query, content);

        let left = vec!["safe, fast, productive."];

        assert_eq!(right, left)
    }

    #[test]
    fn two_results() {
        let query = "public";
        let content =
            "public class Main {
            public static void main(String... args) {
                System.out.println('Hello, World!');
            }
        }";

        let left = search(query, content);
        let right = vec!["public class Main {", "public static void main(String... args) {"];

        assert_eq!(left, right)
    }

    #[test]
    fn case_sensitive() {
        let query = "Public";
        let content =
            "public class Main {
            public static void main(String... args) {
                System.out.println('Hello, World!');
            }
        }";

        let left = search(query, content);
        let right: Vec<&str> = vec![];

        assert_eq!(left, right)
    }

    #[test]
    fn case_insensitive() {
        let query = "Public";
        let content =
            "public class Main {
            public static void main(String... args) {
                System.out.println('Hello, World!');
            }
        }";

        let left = search_case_insensitive(query, content);
        let right: Vec<&str> = vec!["public class Main {", "public static void main(String... args) {"];

        assert_eq!(left, right)
    }
}