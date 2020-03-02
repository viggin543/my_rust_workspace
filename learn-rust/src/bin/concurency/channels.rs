use rand::random;
use std::sync::mpsc; // MPSC stands for multiple consumer single receiver
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

pub fn simple_blocking_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); //blocking. try_recv is async
    println!("Got: {}", received);
}

pub fn iterate_a_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn mpsc_example() {
    let (tx, rx) = mpsc::channel();
    tx.produce_some_strings().produce_some_strings();

    for v in rx {
        println!("got  {}", v);
    }
}

trait Spawn {
    fn produce_some_strings(self) -> Sender<String>;
}

impl Spawn for Sender<String> {
    fn produce_some_strings(self) -> Sender<String> {
        let rand = (random::<i8>() & 2).to_string();
        let ret = self.clone();
        thread::spawn(move || {
            let vals = vec![
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "a", "b", "c", "d", "e",
                "f", "g", "h", "i", "j", "k", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j",
                "k", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "a", "b", "c", "d",
                "e", "f", "g", "h", "i", "j", "k",
            ];
            for v in vals {
                self.send(String::from(v) + &rand[..]).unwrap();
            }
        });
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_in_action() {
        simple_blocking_channel();
        assert!(false);
    }

    #[test]
    fn channel_iterator() {
        iterate_a_channel();
        assert!(false);
    }

    #[test]
    fn multiple_producers_single_consumer_example() {
        mpsc_example();
        assert!(false);
    }
}
