struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_hi(&self) {
        println!("Hi, i'm {} and i'm {} year{} old.",self.name, self.age ,
            if self.age > 1 { "s" }  else { "" });
    }
}

fn main() {
    let john_smith = Person { name: String::from("John Smith"), age: 32 };
    john_smith.say_hi();
}
