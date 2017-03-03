use diesel;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::pg::PgConnection;
use models::*;
use super::schema::sysinfo;

//TODO impl FromData for entry to make the Json respone closer to the standard
pub fn create_entry(conn: &PgConnection, entry: &SysInfoData) -> Result<SysInfo, Error> {
    diesel::insert(entry)
        .into(sysinfo::table)
        .get_result(conn)
}

// get_latest returns the entry with the highest id in the table - i.e. the latest
// TODO is it better to use datetime?
pub fn get_latest(conn: &PgConnection) -> Result<SysInfo, Error> {
    sysinfo::table.order(sysinfo::id.desc()).limit(1).first::<SysInfo>(conn)
}

pub fn get_entry(conn: &PgConnection, id: i32) -> Result<SysInfo, Error> {
    sysinfo::table.find(id)
        .first::<SysInfo>(conn)
}

pub fn get_entries(conn: &PgConnection) -> Result<Vec<SysInfo>, Error> {
    sysinfo::table.load::<SysInfo>(conn)
}
