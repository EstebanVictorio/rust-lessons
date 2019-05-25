#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;


struct Message {
    contents: String,
}

#[get("/")]
fn home() -> String {
    String::from("Home page")
}

fn main() {
    rocket::ignite().mount("/",routes![home]).launch();
}

