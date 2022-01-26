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
			Arg::with_name("no-sync")
				.help("Don't synchronize services before other things")
				.short("ns")
				.long("no-sync")
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
	
	// let stdout = console::Term::stdout();
	// let stderr = console::Term::stderr();
	// use console::style;
	
	// sync stuff first, then do other stuff
	if !app.is_present("no-sync") {
		self::commands::synchronize(&connection).await;
	}

	if app.values_of("get").is_some() {
		// wants to get a service and display it. can be put in multiple times
		let ids = app.values_of("get").unwrap();
		for id_str in ids {
			let id = id_str.parse::<i32>();
			if id.is_ok() {
				let service = self::database::fetch_service(&connection, &id.unwrap()).unwrap();
				self::functions::display_service(&service);
			} else {
				// write a line to stderr that the id is invalid
				// stderr.write_line(
				// 	style(&"Flag {value} is an invalid number"
				// 			.replace("{value}", id_str)).red().toString()
				// ).ok();
			}
		}
	}
	if app.is_present("get-all") {
		// wants to get all services and display them
		let services = self::database::fetch_services(&connection);
		if services.is_ok() {
			let services = services.unwrap();
			for service in services {
				self::functions::display_service(&service);
			}
		}
	}
}
