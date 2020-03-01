pub fn add_t2o(x:u32) -> u32 {
    x+2
}

/// # this is a documentatino of add two
///
/// ```
/// use mygrep::add_two::add_two;
/// let two = add_two(1,2);
/// assert_eq!(4,two)
/// ```
pub fn add_two(a: u32, y :u32) -> u32 {
    a+y
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let val = add_t2o(2);
        assert_eq!(4,val);
    }
}