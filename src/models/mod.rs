pub mod user;
mod schema;
use diesel::pg::PgConnection;
use std::env;
use diesel::r2d2::{ConnectionManager, Pool};

pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}