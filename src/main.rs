#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;
use self::models::*;
use self::schema::services::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
fn main() {
	let connection = establish_connection();
	let results = services
		.load::<Service>(&connection)
		.expect("Error loading services");
	
	println!("Displaying {} posts", results.len());
    for service in results {
        println!("ID {}: {}", service.id, service.secret);
    }
}