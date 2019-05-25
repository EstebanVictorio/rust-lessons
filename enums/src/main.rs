use std::fmt::Display;

enum ECMA {
    ES5,
    ES6,
}

enum ECMA_ADV {
    ES5(String),
    ES6(String)
}

fn main() {
    let standard = ECMA::ES5;
    let std_adv = ECMA_ADV::ES5(String::from("Using std ES5."));
    match standard {
        ECMA::ES5 => println!("You are using the ES5 standard"),
        ECMA::ES6 => println!("You are using the ES6 standard"),
    }

    match std_adv {
        ECMA_ADV::ES5(String) => println!("You are using the ES5 standard"),
        ECMA_ADV::ES6(String) => println!("You are using the ES6 standard"),
    }
}
