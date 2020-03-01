// multiple futures onto the same thread.
use futures::executor::block_on;

async fn hello_world() -> String {
    println!("hello, world!");
    String::from("Banana")
}

fn main() {
    let future = hello_world(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
    block_on(another_async());
}

async fn another_async() {
    let x =  hello_world().await;
    println!("await x: {}",x);
}