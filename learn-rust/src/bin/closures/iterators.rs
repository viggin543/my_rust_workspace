pub fn da() {
    println!("da");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mut_ref_iter_map() {
        let mut v = vec!["A".to_string(), "B".to_string()];
        for s in v.iter_mut() {
            *s = s.to_lowercase()
        }
        println!("{:?}", v);
        assert_ne!(v, v);
    }

    #[test]
    fn lazy_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        let v1 = v1.iter().map(|x| x + 1);
        for i in v1 {
            println!("map babye {}", i);
        }
        assert_eq!(0, 1);
    }

    #[test]
    fn mut_iter() {
        let v = vec!["a".to_string(), "b".to_string()];
        for s in v.into_iter() {
            // s has type String, not &String
            println!("{}", s);
        }
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        // sum consumes the iterator. and it cant be used any more after call to sum
        assert_eq!(total, 6);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}

// custom iterator

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32; // associated type
                     // we need this in order to have a single iterator for Counter.
                     // otherwise we would have needed to specify a type parameter to every call to next method
                     // next::<u32>(). which sucks
                     //https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

// this wont compile.
// no operator overloading
//fn da1() {}
//fn da1(u: i8) {}
