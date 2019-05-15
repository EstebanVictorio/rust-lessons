use std::io;


fn main() {
    println!("Please write your number:");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Wrong input!");
    let guess : u32 = guess
        .trim()
        .parse()
        .expect("Not a number!");
    println!("Factorial of {} is: {}",guess, factorial(guess));
    
}


fn factorial(number: u32) -> u32 {
    if number == 1 { number } else { number * factorial(number - 1) }
}