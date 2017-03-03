use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket::request::{Outcome, FromRequest};
use rocket::Outcome::{Success, Failure, Forward};
use rocket::{Request, State};
use rocket::http::Status;
use std::ops::Deref;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub const DATABASE_URL: &'static str = dotenv!("DATABASE_URL");

pub fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}

pub struct Conn(r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for Conn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Conn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<Conn, ()> {
        let pool =
            match <State<Pool> as FromRequest>::from_request(request) {
                Success(pool) => pool,
                Failure(e) => return Failure(e),
                Forward(_) => return Forward(()),
            };
        match pool.get() {
            Ok(conn) => Success(Conn(conn)),
            Err(_) => Failure((Status::InternalServerError, ())),
        }
    }
}
