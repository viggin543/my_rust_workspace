use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct MyError(String);

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let len = no_error()?.len();
    println!("size {}", len);

    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", MyError("sa".to_string()));
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn no_error() -> Result<String, Box<dyn Error>> {
    Ok("banana".to_string())
}