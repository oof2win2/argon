// #[macro_use]
extern crate dotenv;

pub mod models;
pub mod database;
pub mod functions;

use rusqlite::{Connection};
use dotenv::dotenv;
use self::database::{init_db,createService,fetchService,removeService};
use self::functions::{encode, decode, generate_totp};

pub fn establish_connection(path: &str) -> Connection {
	return Connection::open(&path)
			.expect("DB Connection not opened successfully");
}

fn main() {
	dotenv().ok();
	let connection = establish_connection(&dotenv::var("DATABASE_URL").unwrap_or(String::from("./database.sqlite")));
	init_db(&connection);

	let input = decode("HXDMVJECJJWSRB3HWIZR4IFUGFTMXBOZ");
	if input.is_none() {
		println!("decoding failed");
		std::process::exit(1);
	}
	createService(&connection, input.unwrap());
	let res = fetchService(&connection, &1);
	
	if res.is_some() {
		let safe = res.unwrap();
		for code in &safe.secret {
			print!("{:#}", code)
		}
		println!("is the hex for {}", safe.id);
		println!("{} is the secret, the TOTP code is {}", encode(&safe.secret), generate_totp(&safe.secret))
	} else {
		println!("No service found");
	}

}