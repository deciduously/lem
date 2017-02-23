use chrono;
use super::schema::sysinfo;
use serde_json;
use std::process::Command;

#[derive(Queryable)]
pub struct SysInfo {
    pub id: i32,
    pub datetime: chrono::NaiveDateTime,
    pub uname: String,
    pub uptime: String,
}

#[derive(Insertable)]
#[table_name="sysinfo"]
pub struct NewSysInfo<'a> {
    pub datetime: chrono::NaiveDateTime,
    pub uname: &'a str,
    pub uptime: &'a str,
}


impl SysInfo {
    pub fn display(&self) -> String {
        format!("RED - {}", self.get_json())
    }
    pub fn get_json(&self) -> serde_json::Value {
        return json!({ "uname": self.uname,
                "uptime": self.uptime});
    }
}

// fn get_avail_updates() -> u32 {
// let updates = Command::new("checkupdates")
// }

// maybe make macros
pub fn get_uname() -> String {
    let uname = Command::new("uname")
        .arg("-orm")
        .output()
        .expect("uname failed");
    String::from_utf8(uname.stdout).unwrap()
}

pub fn get_uptime() -> String {
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("could not retrieve uptime");
    String::from_utf8(uptime.stdout).unwrap()
}
// MAYBE DELETE?
pub fn strip(s: String) -> String {
    let mut ret = s.clone();
    match ret.pop() {
        Some('\n') => ret,
        _ => s,
    }

}
