use chrono;
use super::schema::sysinfo;
use std::process::Command;

#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
pub struct SysInfo {
    pub id: i32,
    pub datetime: chrono::NaiveDateTime,
    pub uname: String,
    pub uptime: String,
}

#[derive(Insertable, Deserialize)]
#[table_name="sysinfo"]
pub struct SysInfoData {
    pub datetime: chrono::NaiveDateTime,
    pub uname: String,
    pub uptime: String,
}

impl Default for SysInfoData {
    fn default() -> Self {
        Self::new()
    }
}

impl SysInfoData {
    pub fn new() -> SysInfoData {
        let now = chrono::UTC::now().naive_utc();
        SysInfoData {
            datetime: chrono::NaiveDateTime::new(now.date(), now.time()),
            uname: strip(get_uname()),
            uptime: strip(get_uptime()),
        }
    }
}

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

pub fn strip(s: String) -> String {
    let mut ret = s.clone();
    match ret.pop() {
        Some('\n') => ret,
        _ => s,
    }
}
