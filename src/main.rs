#![feature(plugin)]
#![plugin(rocket_codegen)]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rocket;

mod sysinfo;

#[get("/")]
fn index() -> String {
    let sysinfo = sysinfo::SysInfo::new();
    sysinfo.display()
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
