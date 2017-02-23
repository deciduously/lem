use std::process::Command;
use serde_json::Value;

pub struct SysInfo {
    pub uname: String,
    pub uptime: String,
}

impl SysInfo {
    pub fn new() -> SysInfo {
        SysInfo {
            uname: strip(get_uname()),
            uptime: strip(get_uptime()),
        }
    }
    pub fn display(&self) -> String {
        format!("RED - {}", self.get_json())
    }
    pub fn get_json(&self) -> Value {
        return json!({ "uname": self.uname,
                "uptime": self.uptime});
    }
}

// fn get_avail_updates() -> u32 {
// let updates = Command::new("checkupdates")
// }

//maybe make macros
fn get_uname() -> String {
    let uname = Command::new("uname")
        .arg("-orm")
        .output()
        .expect("uname failed");
    String::from_utf8(uname.stdout).unwrap()
}

fn get_uptime() -> String {
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("could not retrieve uptime");
    String::from_utf8(uptime.stdout).unwrap()
}
fn strip(s: String) -> String {
    let mut ret = s.clone();
    match ret.pop() {
        Some('\n') => ret,
        _ => s,
    }

}
