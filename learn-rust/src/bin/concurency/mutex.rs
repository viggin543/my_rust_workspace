use core::borrow::Borrow;
use std::sync::Mutex;
use std::sync::{atomic, Arc};
use std::thread;

pub fn this_is_a_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // lock blocks untiil current thread gets the lock
        *num = 6;
    }

    println!("m = {:?}", m); // prints 6
}

fn complex_example() {
    let counter = Arc::new(Mutex::new(0));
    // atomic reference counting
    // Rc<> ownership is not thread safe and cant be passed between threads since it does not implement the Send marker trait !
    //  Sync marker trait indicates that many threads can hold a reference to an object. . https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html#extensible-concurrency-with-the-sync-and-send-traits
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // how .lock is possible on Atc ? extension func ?
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", (*counter).lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutex_test() {
        this_is_a_mutex();
        assert!(false);
    }
    #[test]
    fn complex_example_in_action() {
        complex_example();
        assert!(false);
    }
}
