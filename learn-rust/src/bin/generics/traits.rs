use std::fmt::Debug;
use std::fmt::Display;
use std::string::ToString;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn hump(&self) {
        println!("hump");
    }
    fn da() {
        println!("da");
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        "NewsArticle author".to_string()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        "tweet author".to_string()
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyBound<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyPlusSyntax(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyWow<T: Summary + Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {3}
fn some_function_with_where<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{3}


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_clone<T>(list: &[T]) -> T
    where T: PartialOrd + Clone
{
    let mut largest = list[0].clone();
    for item in list.clone().iter() {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}


fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list.iter() {
        if item > max {
            max = item
        }
    }
    max
}

struct Pair<T>{
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

trait Jump {
    fn jump(&self) -> String;
}


#[derive(Debug)]
struct ExtendMe(String);
//blanket impl. implement ToString trait for traits that implement the Display trait
// extension function !!!!
impl<T: Debug> Jump for T {
    fn jump(&self) -> String {
        unimplemented!()
    }
}


fn is_this_an_extension_function(){
    let e = ExtendMe("da".to_string());
    e.jump();
}