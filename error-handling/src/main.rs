use std::io;
use std::io::Read;
use std::fs::File;
fn main() {
    // panic!("Crash and burn"); Panicking commented as a sample
    println!("string: {:?}", read())
}

fn read() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
