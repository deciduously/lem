#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to RED"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
