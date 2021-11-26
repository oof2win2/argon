#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use base32;

// use ootp::constants::*;
use totp_rs::{Algorithm, TOTP};
use std::time::SystemTime;

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
fn generate_totp(secret: &Vec<u8>) -> String {
	let totp = TOTP::new(
		Algorithm::SHA1,
		6,
		1,
		30,
		secret,
	);
	let time = SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH).unwrap()
		.as_secs();
	let otp = totp.generate(time);
	return otp
}

fn main() {
	let input = "DG4ERUOUMURJHI77I4HZW53WUFMRNUNG";
	let result = base32::decode(base32::Alphabet::RFC4648{padding: true}, input).unwrap();
	for i in 1..result.len() {
		print!("{:#x} ", result[i]);
	}
	println!("");
	println!("{}", generate_totp(&result));
}