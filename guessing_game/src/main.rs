extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering::Less;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Equal;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("The secret number is: {}",secret_number);
    loop {
        println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);
    match guess.cmp(&secret_number) {
        Less => println!("Too small!"),
        Greater => println!("Too big!"),
        Equal => {
            println!("You win!");
            break;
        }
    }
    }
}
