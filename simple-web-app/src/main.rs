#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate restaurant;
use restaurant::Hosting;

struct Message {
    contents: String,
}

#[get("/")]
fn home() -> String {
    Hosting::add_to_waiting_list();
    String::from("Home page")
}

fn main() {
    rocket::ignite().mount("/",routes![home]).launch();
}

