mod errors;
use errors::error_handle;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{ //dyn Error -> any kind of error
    let guess = Guess::new(10);
    println!("guess is {}",guess.value());
    println!("guess is {}",guess.value()); // not borowwd since is a primitive

    error_handle();
    Ok(())
}

pub struct Guess {
    value: i32,
}

impl Guess { //custom type
    pub fn new(value: i32) -> Guess { //private constractor !!
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}