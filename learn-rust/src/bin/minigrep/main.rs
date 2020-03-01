use std::env;
use std::fs;
use std::process;
use std::error::Error;
use ::mygrep::config::{self,Config};
use mygrep::search::{search, search_case_insensitive};

fn main()  {
    let conf =  config::parse_args().unwrap_or_else(|e| {
        eprintln!("Application error: {}", e);
        process::exit(1);
    });
    if let Err(e) = run(conf) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}


fn run(config: Config) -> Result<(), Box<dyn Error>> { // dynamic type that implements the Error trait
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };


    for x in results.iter() { // why does this work ? ref to ref
        println!("{}",x)
    }

    Ok(())
}

