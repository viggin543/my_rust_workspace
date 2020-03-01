use crate::traits::{Tweet, Summary, notify};
use crate::lifetimes::call_a_fn_with_lifetime;

mod gen;
mod traits;
mod lifetimes;
fn main() {
    println!("learn generics-> https://doc.rust-lang.org/book/ch10-01-syntax.html");
    gen::point();
    let tvit = Tweet {
        content: "content".to_string(),
        reply: false,
        retweet: false,
        username: "boris".to_string(),
    };
    tvit.hump();
    println!("{}",tvit.summarize());
    Tweet::da();
    notify(tvit);
    call_a_fn_with_lifetime();

//    take_ownership(tvit);
}


fn take_ownership(t: Tweet){
    println!("{:?}",t);
}
