use diesel::result::Error;
use diesel::pg::PgConnection;
use models::*;
use diesel::prelude::*;
use super::schema::sysinfo;


pub fn get_entry(conn: &PgConnection, id: i32) -> Result<SysInfo, Error> {
    sysinfo::table.find(id)
        .first::<SysInfo>(conn)
}

pub fn get_entries(conn: &PgConnection) -> Result<Vec<SysInfo>, Error> {
    sysinfo::table.load::<SysInfo>(conn)
}
