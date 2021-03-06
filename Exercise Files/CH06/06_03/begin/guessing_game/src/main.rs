extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    
    println!("Please input your guess..");

    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("Secret number is = {}", secret_number);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("failed to read line!");

    println!("You guessed: {}", guess);
}
