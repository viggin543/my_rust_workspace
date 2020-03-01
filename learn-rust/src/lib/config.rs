use std::env;
use std::str::FromStr;
use std::env::Args;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename,case_sensitive })
    }
}

pub fn parse_args() -> Result<Config, &'static str> {
    let args: Vec<String> = env::args().collect();
    Config::new(env::args())
}



