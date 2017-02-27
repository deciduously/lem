#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate chrono;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod db;
mod error;
mod schema;
mod models;
mod sysinfo;

use db::DB;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use error::Error as ApiError;
use rocket_contrib::JSON;
use sysinfo::{get_entries, get_entry};


use self::models::{SysInfo, NewSysInfo};
pub fn create_entry(conn: &PgConnection) -> SysInfo {
    use schema::sysinfo;
    let now = chrono::UTC::now().naive_utc();

    let new_entry = NewSysInfo {
        datetime: chrono::NaiveDateTime::new(now.date(), now.time()),
        uname: models::strip(models::get_uname()),
        uptime: models::strip(models::get_uptime()),
    };

    diesel::insert(&new_entry)
        .into(sysinfo::table)
        .get_result(conn)
        .expect("Error saving new entry")
}

#[get("/")]
fn index(db: DB) -> String {

    let new_entry = create_entry(db.conn());
    format!("Successfully inserted record #{}  - uname: {} uptime: {} into the table\n{}",
            new_entry.id,
            new_entry.uname,
            new_entry.uptime,
            new_entry.datetime)
}


#[get("/entries", format="application/json")]
fn entries(db: DB) -> Result<JSON<Vec<SysInfo>>, ApiError> {
    let entries = get_entries(db.conn())?;
    Ok(JSON(entries))
}

#[get("/entry/<id>", format="application/json")]
fn entry(db: DB, id: i32) -> Result<JSON<SysInfo>, ApiError> {
    let entry = get_entry(db.conn(), id)?;
    Ok(JSON(entry))
}

fn main() {
    rocket::ignite().mount("/", routes![index, entry, entries]).launch();
}
