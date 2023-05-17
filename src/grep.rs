use std::{env, fs, process::exit};

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() > 3 {
            println!("Only Query and Filename are allowed. Other args will be ignored");
        }
        let query = match args.get(1) {
            Some(q) => q.clone(),
            None => return Err("No Query Given"),
        };
        let filename = match args.get(2) {
            Some(f) => f.clone(),
            None => return Err("No Filename Given"),
        };

        Ok(Config { query, filename })
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
    println!("Grep Remade");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let content = get_file_content(&config).unwrap_or_else(|err| {
        println!("Problem reading file: {}", config.filename);
        println!("{}", err);
        exit(1);
    });

    let results = search(config.query.as_str(), content.as_str());

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

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<Line<'a>> {
    let mut results = vec![];
    for (index, line) in content.lines().enumerate() {
        if line.contains(query) {
            results.push(Line {
                index: index + 1,
                text: line,
            });
        }
    }

    results
}
