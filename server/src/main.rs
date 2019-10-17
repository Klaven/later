#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

pub mod controllers;

#[get("/")]
fn index() -> &'static str {
    "Bye."
}

fn main() {
    println!("Hello, world!");
    rocket::ignite().mount("/", controllers::get_routes()).launch();
}
