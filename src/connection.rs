use dotenv::dotenv;
use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;

pub type Pool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::new(database_url());
    Pool::builder()
        .build(manager)
        .expect("Could not create database connection pool")
}

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    PgConnection::establish(&database_url())
        .expect(&format!("Error connecting to {}", database_url()))
}
