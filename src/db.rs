use r2d2;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use iron::typemap::Key;

pub type PostgresPool = r2d2::Pool<PostgresConnectionManager>;
pub type PostgresConnection = r2d2::PooledConnection<PostgresConnectionManager>;

pub struct PostgresDB;
impl Key for PostgresDB { type Value = PostgresPool;}

macro_rules! get_pg_connection {
    ($req:expr) => (match $req.get::<persistent::Read<db::PostgresDB>>() {
        Ok(pool) => match pool.get() {
            Ok(conn) => conn,
            Err(_) => {
                println!("Couldn't get a connection to pg!");
                return Ok(Response::with((status::InternalServerError)));
            }
        },
        Err(_) => {
            println!("Couldn't get the pg pool from the request!");
            return Ok(Response::with((status::InternalServerError)));
        }
    })
}

pub fn get_pool(uri: &str) -> PostgresPool {
    let manager = PostgresConnectionManager::new(uri, TlsMode::None).unwrap();

    r2d2::Pool::new(r2d2::Config::default(), manager).unwrap()
}
