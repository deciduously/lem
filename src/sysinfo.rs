use models::*;

pub fn new() -> SysInfo {
    SysInfo {
        uname: strip(get_uname()),
        uptime: strip(get_uptime()),
    }
}
pub fn display(&self) -> String {
    format!("RED - {}", self.get_json())
}
pub fn get_sysinfo() -> Result<Sysinfo, Error> {
    return json!({ "uname": self.uname,
            "uptime": self.uptime});
}

// this pattern looks ripe for learning how macros work, Ben
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