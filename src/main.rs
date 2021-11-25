#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use std::str;

// use ootp::constants::*;
use ootp::totp::{CreateOption, Totp};
use data_encoding::BASE32;

pub mod models;
pub mod schema;
use self::models::*;
use self::schema::services::dsl::services;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

/**
 * Generate a TOTP string from a base32 encoded secret
 */
fn generate_totp(secret: &str) {
	let totp = Totp::secret(secret, CreateOption::Default);
	let otp = totp.make();
	println!("OTP is {}", otp);
}

fn main() {
	// let connection = establish_connection();
	// let results = services
	// 	.load::<Service>(&connection)
	// 	.expect("Error loading services");
	
	// println!("Displaying {} posts", results.len());
    // for service in results {
    //     println!("ID {}: {}", service.id, service.secret);
    // }
	
	let result = match BASE32.decode("IEQHG5DSN5XGOIDTNBQXEZLEEBZWKY3SMV2A====".as_bytes()) {
		Ok(data) => {
			data
		}
		Err(error) => {
			println!("{}", error);
			[0].to_vec()
		}
	};
	let strres = str::from_utf8(&result).unwrap();
	println!("{}", strres);
	generate_totp(&strres);
}