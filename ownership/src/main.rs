use std::io;

fn main() {

    let mut first_word = String::new();
    println!("Type an statement:");
    io::stdin()
        .read_line(&mut first_word)
        .expect("Not a valid string!");
    let first_word = get_first_word(&first_word);
    println!("The first word is: {}", first_word);
    
}


fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[0..]
}
