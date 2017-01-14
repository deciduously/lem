use std::process::Command;
use std::string::String;

#[derive(Debug)]
pub struct SysInfo {
    uptime: String,
}

impl SysInfo {
    pub fn new() -> SysInfo {
        SysInfo { uptime: get_uptime() }
    }
    pub fn display(&self) -> String {
      format!("RED - {}", self.uptime)
    }
}

fn get_uptime() -> String {
    let uptime = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("could not retrieve uptime");
    String::from_utf8(uptime.stdout).unwrap()
}