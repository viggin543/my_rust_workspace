fn main() {

}

//derivable traits
//https://doc.rust-lang.org/book/appendix-03-derivable-traits.html
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


///https://doc.rust-lang.org/book/ch11-02-running-tests.html
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn expensive_test() { //cargo test -- --ignored
        // code that takes an hour to run
    }

    #[test]
    fn test_that_returns_error() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // ASSERT PANIC MESSAGE
    fn should_panic() {
        panic!("Guess value must be less than or equal to 100")
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }


    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn testing_baby() {
        assert_ne!(1,2);
    }

}
