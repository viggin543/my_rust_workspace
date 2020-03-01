use rand::Rng;
use std::cmp::Ordering;
use std::io;

mod rect;
use crate::rect::Point;



fn main() {
    play_with_point();
    launch_game();
}

fn play_with_point(){
    let x = Point {x:1,y:3};
    println!("da {:?} ",x);
}

fn launch_game(){
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("{} + {} = {}",0.1,3,0.1+3.0);
    game_loop(secret_number);
}


fn game_loop(secret_number: u32){
loop {
    let mut guess = String::new();
    println!("Please input your guess.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
     };
     
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}

}