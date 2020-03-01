use std::fs;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

pub fn error_handle() {
    // set to get stacktrace -> export RUST_BACKTRACE=1
    let v = vec![1, 2, 3];
    if let Err(e) = read_username_from_file() {
        println!("oops.. {}",e)
    }
    read_username_from_file_v2();
    read_username_from_file_v3();
    file_error();
    v[99];
}


fn file_error() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("creating hello.txt");
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("{}", "deleting hello.txt");
    if let Ok(meta) = f.metadata() {
        fs::remove_file("hello.txt");
    }
}


fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // error propogate operator ( like a throw )
    let mut s = String::new();
    f.read_to_string(&mut s)?; // can be used only in a functino that returns Result or option...
    Ok(s)
}

fn read_username_from_file_v2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; // error propogation chaining. VERY NICE !!!

    Ok(s)
}

fn read_username_from_file_v3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}