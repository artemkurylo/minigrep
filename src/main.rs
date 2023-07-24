use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args()
        .collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    run(config)
}

struct Config {
    regex: String,
    path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 3 {
            return Err("too many arguments");
        }

        let regex = args[1].clone();
        let path = args[2].clone();

        Ok(Self { regex, path })
    }
}

fn run(config: Config) {
    let file_content = fs::read_to_string(config.path)
        .expect("can't read the file content");

    let lines: Vec<&str> = file_content
        .lines()
        .collect();



    for line in lines {
        if line.contains(&config.regex) {
            println!("{}", line)
        }
    }
}