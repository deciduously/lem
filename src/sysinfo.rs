use std::process::Command;

pub struct SysInfo {
    pub uptime: String,
}

impl SysInfo {
    pub fn new() -> SysInfo {
        SysInfo { uptime: get_uptime() }
    }
}

fn get_uptime() -> String {
    let uptime = Command::new("uptime")
        .arg("p")
        .output()
        .expect("could not retrieve uptime");
    format!("{}", uptime.status)
}