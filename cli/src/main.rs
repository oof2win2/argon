extern crate tokio;
extern crate dotenv;
extern crate clap;

pub mod backend;
pub mod database;
pub mod functions;
pub mod models;
pub mod commands;

use self::database::{init_db};
use dotenv::dotenv;
use rusqlite::Connection;
use clap::{App, Arg, crate_authors, crate_version};
// use clap::args::any_arg::AnyArg;

pub fn establish_connection(path: &str) -> Connection {
    return Connection::open(&path).expect("DB Connection not opened successfully");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let connection = establish_connection(
        &dotenv::var("DATABASE_URL").unwrap_or(String::from("./database.sqlite")),
    );
    init_db(&connection);

	let app = App::new("argon")
		.author(crate_authors!())
		.version(crate_version!())
		.about("An app for OTP code management")
		.arg(
			Arg::with_name("sync")
				.help("Synchronize services")
				.short("s")
				.long("sync")
		)
		.arg(
			Arg::with_name("get-all")
				.help("Get OTP codes for all services")
				.long("get-all")
			)
		.arg(
			Arg::with_name("get")
				.help("Get OTP codes for a defined service")
				.short("g")
				.long("get")
				.takes_value(true)
				.value_name("ID")
		)
		.get_matches();
	if app.value_of("get").is_some() {
		let value = app.value_of("get").unwrap();
		println!("value of get is {}", value);
	}
	if app.is_present("get-all") {
		println!("get-all is present");
	}
	if app.is_present("sync") {
		self::commands::synchronize(&connection);
	}
}
