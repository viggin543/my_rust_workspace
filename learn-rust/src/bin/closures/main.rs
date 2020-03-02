use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

mod iterators;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    clousure_borrow();
    closure_move();
    iterators::da();
}

fn inner_func() {
    let x = 4;
    //    inner func cant access X
    //    fn equal_to_x(z: i32) -> bool { z == x }
    //    let y = 4;
    //    assert!(equal_to_x(y));
}
fn closure_move() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    //    println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

fn clousure_borrow() {
    let x = 4;
    let equal_to_x = |z| z == x; // ref to x is borrowed by closure
    let y = 4;
    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cacher = Cacher::new(|num| {
        // a closure that implements FN trait
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

struct Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash + Display,
{
    calculation: T,
    value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
where
    T: Fn(U) -> U,
    U: Copy + Eq + Hash + Display,
{
    fn new(calculation: T) -> Cacher<T, U> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> U {
        let res = &(self.calculation)(arg);
        self.value.entry(arg).or_insert(*res);
        println!("len {}", self.value.len());

        for (k, v) in self.value.iter() {
            println!("k {} v {}", k, v);
        }
        *res
    }
}
