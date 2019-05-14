use std::io;
fn main() {
    println!("Please write your number:");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Invalid input!");
    
    let number: u32 = number
        .trim()
        .parse()
        .expect("Couldn't parse!");

    println!("The fibonacci result for {} is: {}", number ,fibonacci(number));
}


fn fibonacci(number: u32) -> u32 {
    if number == 0 || number == 1 {
        number
    } else {
        fibonacci(number - 1) + fibonacci(number -2)
    }
}
