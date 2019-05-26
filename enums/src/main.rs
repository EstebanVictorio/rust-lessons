use std::fmt::Display;

enum ECMA {
    ES5,
    ES6,
}

enum ECMA_ADV {
    ES5(String),
    ES6(String)
}

#[derive(Debug)]
enum Message {
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let standard = ECMA::ES5;
    let std_adv = ECMA_ADV::ES5(String::from("Using std ES5."));
    let msg = Message::Quit;
    let option_1: Option<u32> = Some(12);
    let option_2 : Option<u32> = None;
    msg.call();
    println!("Option 1: {}", option_1.unwrap());
    println!("Option 2: {}", option_2.unwrap()); // This panics, since we cannot unwrap a value from 'None'
    match standard {
        ECMA::ES5 => println!("You are using the ES5 standard"),
        ECMA::ES6 => println!("You are using the ES6 standard"),
    }

    match std_adv {
        ECMA_ADV::ES5(String) => println!("You are using the ES5 standard"),
        ECMA_ADV::ES6(String) => println!("You are using the ES6 standard"),
    }
}
