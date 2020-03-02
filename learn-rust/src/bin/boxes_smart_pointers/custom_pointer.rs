use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
//DerefMut Deref trait and Mutability
//https://doc.rust-lang.org/book/ch15-02-deref.html#how-deref-coercion-interacts-with-mutability
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    } // a type that implements deref can use deref coercion. s
      // o you can access wrapped type without explicitly using the * deref operator
      // like here: mutex.rs:25
      // notice -> counter.lock()
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox!");
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_coercion() {
        let m = MyBox::new(String::from("Rust"));
        hello(&m); // pass Mybox<String> receives &str
        hello(&(*m)[..]); // coercion instead of...
        drop(m); // release memmory
        assert_ne!(1, 1);
    }

    #[test]
    fn test_box() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
}
