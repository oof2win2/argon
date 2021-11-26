#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

pub mod models;
pub mod database;
pub mod functions;
pub mod pages;

use rusqlite::{Connection};
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use r2d2_sqlite::SqliteConnectionManager;
use self::database::init_db;

pub fn establish_connection(path: &str) -> Connection {
	return Connection::open(&path)
			.expect("DB Connection not opened successfully");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();

	// pooling for SQLite connections
	let manager = SqliteConnectionManager::file(&dotenv::var("DATABASE_URL").unwrap_or(String::from("./database.sqlite")));
	let pool = r2d2::Pool::builder()
		.max_size(16)
		.build(manager)
		.unwrap();
    let connection = &pool.get().unwrap();
	init_db(&connection);

    HttpServer::new(move || {
		App::new()
			.data(pool.clone())
			.service(
				web::resource("/services").route(web::get().to(self::pages::getServices))
			)
			.default_service(
				web::resource("")
					.route(web::get().to(self::pages::p404))
			)
	})
        .bind(&dotenv::var("API_URL").unwrap_or(String::from("127.0.0.1:8080")))?
        .run()
        .await
}