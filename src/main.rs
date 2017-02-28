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
use error::Error as ApiError;
use rocket_contrib::JSON;
use rocket::response::status::Created;
use sysinfo::{create_entry, get_entries, get_entry, get_latest};

#[get("/")]
fn index(db: DB) -> Result<String, ApiError> {
    // TODO handle empty table case
    let latest = get_latest(db.conn())?;
    Ok(format!("Most recent record: #{}  - uname: {} uptime: {} into the table\nCreated @ {}\n",
               latest.id,
               latest.uname,
               latest.uptime,
               latest.datetime))
}

#[get("/entries", format="application/json")]
fn entries_get(db: DB) -> Result<JSON<Vec<SysInfo>>, ApiError> {
    let entries = get_entries(db.conn())?;
    Ok(JSON(entries))
}

#[get("/entry/<id>", format="application/json")]
fn entry_get(db: DB, id: i32) -> Result<JSON<SysInfo>, ApiError> {
    let entry = get_entry(db.conn(), id)?;
    Ok(JSON(entry))
}

use self::models::{SysInfo, SysInfoData};
#[post("/entries", format="application/json")]
fn entry_create(db: DB) -> Result<Created<JSON<SysInfo>>, ApiError> {
    let entry = create_entry(db.conn(), &SysInfoData::new())?;
    let url = format!("/entry/{}", entry.id);
    Ok(Created(url, Some(JSON(entry))))
}

fn main() {
    rocket::ignite().mount("/", routes![index, entry_get, entries_get, entry_create]).launch();
}
