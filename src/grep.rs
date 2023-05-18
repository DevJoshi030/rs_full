use regex::Regex;
use std::{env, fs, process::exit};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub ignore_case: bool,
}

impl Config {
    fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() > 3 {
            println!("Only Query and Filename are allowed. Other args will be ignored");
        }

        args.next();
        let query = match args.next() {
            Some(q) => q,
            None => return Err("No Query Given"),
        };
        let filename = match args.next() {
            Some(f) => f.clone(),
            None => return Err("No Filename Given"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            filename,
            ignore_case,
        })
    }
}

#[derive(Debug)]
pub struct Line<'a> {
    pub index: usize,
    pub text: &'a str,
}

impl<'a> PartialEq for Line<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.text.eq(other.text)
    }
}

pub fn run() {
    println!("Grep Remade!!!");

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if config.ignore_case {
        println!("Ignoring Case");
    }

    let content = get_file_content(&config).unwrap_or_else(|err| {
        println!("Problem reading file: {}", config.filename);
        println!("{}", err);
        exit(1);
    });

    let results = search(config.query.as_str(), content.as_str(), config.ignore_case);

    if results.len() == 0 {
        println!("No results found!!!");
        exit(0);
    }
    println!("Results: <Line Number>. <Text>");

    for line in results {
        println!("{}. {}", line.index, line.text);
    }
}

pub fn get_file_content(config: &Config) -> Result<String, String> {
    let contents = match fs::read_to_string(&config.filename) {
        Ok(c) => c,
        Err(_) => {
            return Err(format!("No such file `{}`", &config.filename));
        }
    };

    Ok(contents)
}

pub fn search<'a>(query: &'a str, content: &'a str, ignore_case: bool) -> Vec<Line<'a>> {
    let re = generate_regex(query, ignore_case);
    content
        .lines()
        .enumerate()
        .filter(|(_, text)| re.is_match(text))
        .map(|(index, text)| Line {
            index: index + 1,
            text,
        })
        .collect()
}

fn generate_regex(query: &str, ignore_case: bool) -> Regex {
    let mut regex_query = String::new();

    if ignore_case {
        regex_query.push_str(r"(?i:");
    }

    regex_query.push_str(query);

    if ignore_case {
        regex_query.push_str(r")");
    }

    Regex::new(regex_query.as_str()).unwrap()
}
