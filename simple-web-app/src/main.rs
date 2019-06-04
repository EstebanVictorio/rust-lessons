#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate restaurant;

struct Message {
    contents: String,
}

#[get("/")]
fn home() -> String {
    restaurant::serve_order();
    String::from("Home page")
}

fn main() {
    rocket::ignite().mount("/",routes![home]).launch();
}

