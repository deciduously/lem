#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate serde_json;
mod models;

use models::SysInfo;

#[get("/")]
fn index() -> String {
    let sysinfo = SysInfo::new();
    sysinfo.display()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
