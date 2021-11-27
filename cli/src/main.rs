#[macro_use]
extern crate tokio;
extern crate dotenv;

pub mod models;
pub mod database;
pub mod functions;
pub mod backend;

use rusqlite::{Connection};
use dotenv::dotenv;
use self::database::{init_db,createService,fetchService,removeService};
use self::functions::{encode, decode, generate_totp};

pub fn establish_connection(path: &str) -> Connection {
	return Connection::open(&path)
			.expect("DB Connection not opened successfully");
}

#[tokio::main]
async fn main() {
	dotenv().ok();
	let connection = establish_connection(&dotenv::var("DATABASE_URL").unwrap_or(String::from("./database.sqlite")));
	init_db(&connection);
	let result = self::backend::getService(&1).await.ok();
	if result.is_some() {
		let data = result.unwrap();
		if data.is_some() {
			let service = data.unwrap();
			println!("{:?} {}", service.secret, service.id);
		} else {
			println!("no service found")
		}
	}
}