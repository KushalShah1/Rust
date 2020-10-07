extern crate rand;
use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {

    let secret= rand::thread_rng().gen_range(1,101);
    println!("{}",secret);
    loop{
        println!("Please input your guess:");
        let mut guess=String::new();

        io::stdin().read_line(&mut guess)
                .expect("failed to read line");

        println!("You guessed {}",guess);

        let guess:u32=guess.trim().parse().expect("invalid guess");

        match guess.cmp(&secret){
            Ordering::Less=>println!("too small"),
            Ordering::Greater=>println!("too big"),
            Ordering::Equal=>{println!("you win!");break;}
        };
    }
}
