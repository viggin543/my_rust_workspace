use rand;
pub fn invoke(x: i32) -> i32 {
    x + 1
}

/// # Add a random number
/// ## examples
/// ```
/// use add_one::rnd_add;
/// let a = 2;
///  assert_ne!(2,rnd_add(a)); // <- unstable doc test ;)
/// ```
///
pub fn rnd_add(x: i32) -> i32 {
    let random_num: i32 = rand::random();
    x + random_num
}

//&cargo test -p add-one
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(invoke(1), 2);
    }
}
