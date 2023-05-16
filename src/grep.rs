use std::{env, fs, process::exit};

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &str> {
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

pub fn run() {
    println!("Grep Remade");

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args: {}", err);
        exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    find(&config).unwrap_or_else(|err| {
        println!("Problem reading file: {}", config.filename);
        println!("{}", err);
        exit(1);
    });
}

fn find<'a>(config: &Config) -> Result<(), &'a str> {
    let contents = fs::read_to_string(format!("{}", config.filename))
        .unwrap_or_else(|_| return format!("No such file `{}`", config.filename));

    println!("Contents of file: {}", config.filename);
    println!("{}", contents);
    Ok(())
}
