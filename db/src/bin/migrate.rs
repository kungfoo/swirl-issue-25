#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;

use simple_logger::SimpleLogger;

use db::connection::establish_connection;

fn main() {
    embed_migrations!();
    SimpleLogger::new().init().unwrap();

    info!("running db migrations...");
    let connection = establish_connection();
    match embedded_migrations::run(&connection) {
        Ok(_) => info!("Successfully ran all pending migrations."),
        Err(e) => {
            error!("Failed to run migrations due to {}", e);
            panic!("Failed to run migrations.");
        }
    }
}
