use rand::random;

fn main() {
    prin_me();
}

pub fn prin_me() {
    let x = 2;
    println!("opa opa {:?}", x);
    let x = if random() {
        Banana::Something(String::from("asd")) 
    } else {
        Banana::Nothing 
    };

    match x {
        Banana::Something(stri) => println!("patter matching bla {}", stri),
        Banana::Nothing => println!("nothing !!!"),
    }
}

#[derive(Debug)]
enum Banana {
    Nothing,
    Something(String),
}

struct DA {
    opa: String,
    id: u32,
}
