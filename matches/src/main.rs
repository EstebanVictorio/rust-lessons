fn main() {
    let option: Option<u32> = Some(13);
    match option {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        _ => println!("Doesn't match")
    }
}
