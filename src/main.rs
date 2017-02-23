#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate chrono;
extern crate rocket;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

use models::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{SysInfo, NewSysInfo};
pub fn create_entry<'a>(conn: &PgConnection) -> SysInfo {
    use schema::sysinfo;
    let now = chrono::UTC::now().naive_utc();

    let new_entry = NewSysInfo {
        datetime: chrono::NaiveDateTime::new(now.date(), now.time()),
        uname: &models::strip(models::get_uname()),
        uptime: &models::strip(models::get_uptime()),
    };

    diesel::insert(&new_entry)
        .into(sysinfo::table)
        .get_result(conn)
        .expect("Error saving new entry")
}

#[get("/")]
fn index() -> String {
    // let sysinfo = SysInfo::new();
    // sysinfo.display()
    //use schema::sysinfo::dsl::*;

    let connection = establish_connection();
    let new_entry = create_entry(&connection);
    format!("Successfully inserted record #{}  - uname: {} uptime: {} into the table\n{}", new_entry.id, new_entry.uname, new_entry.uptime, new_entry.datetime)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
