use std::io;

fn main() {

    let mut first_word = String::new();
    println!("Type an statement:");
    io::stdin()
        .read_line(&mut first_word)
        .expect("Not a valid string!");
    let first_word = get_first_word(&first_word);
    println!("The first word is: {}", first_word);
    let mut first_word_ownership = String::new();
    println!("Type again:");
    io::stdin()
        .read_line(&mut first_word_ownership)
        .expect("Not a valid string!");
    let first_word_ownership = get_first_word_ownership(first_word_ownership);
    println!("The first word is: {}", first_word_ownership);
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

fn get_first_word_ownership (s: String) -> String {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return s.chars().take(i).collect()
        }
    }
     s
}


