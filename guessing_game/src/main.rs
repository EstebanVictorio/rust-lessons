extern crate rand;
use std::io;
use rand::Rng;
use std::cmp::Ordering::Less;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Equal;

pub struct Guess {
    value: i32
}


impl Guess{
    pub fn new(value: i32) -> Guess {
        if value > 100 || value < 1 {
            panic!("Not allowed outside a range of 1 < n < 100, got {}",value);
        }
        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


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
    let guess: i32 = guess.trim().parse().expect("Please type a number!");
    let guess: Guess = Guess::new(guess);
    println!("You guessed: {}", guess.value);
    match guess.value.cmp(&secret_number) {
        Less => println!("Too small!"),
        Greater => println!("Too big!"),
        Equal => {
            println!("You win!");
            break;
        }
    }
    }
}
