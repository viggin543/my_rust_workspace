use add_one;
//$cargo run -p adder to run this from root dir
fn main() {
    println!("Hello, world! {}", add_one::invoke(3));
}
