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
    let counter = Arc::new(Mutex::new(0)); // atomic reference counting

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
