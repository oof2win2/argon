#[macro_use]
extern crate serde_json;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_sqlite;
extern crate rusqlite;

pub mod database;
pub mod functions;
pub mod models;
pub mod pages;

use self::database::init_db;
use actix_web::{web, App, HttpServer, middleware};
use dotenv::dotenv;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;

pub fn establish_connection(path: &str) -> Connection {
    return Connection::open(&path).expect("DB Connection not opened successfully");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // pooling for SQLite connections
    let manager = SqliteConnectionManager::file(
        &dotenv::var("DATABASE_URL").unwrap_or(String::from("./database.sqlite")),
    );
    let pool = r2d2::Pool::builder().max_size(16).build(manager).unwrap();
    let connection = &pool.get().unwrap();
    init_db(&connection);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
			.wrap(middleware::Logger::default())
			.wrap(middleware::Compress::default())
            .service(
                web::scope("/services")
                    .service(
						web::resource("")
							.route(web::get().to(self::pages::get_services))
							.route(web::post().to(self::pages::create_service))
					)
                    .service(
                        web::scope("/{id}")
							.service(
								web::resource("")
									.route(web::get().to(self::pages::get_service))
									.route(web::delete().to(self::pages::remove_service))
							)
                    ),
            )
            .default_service(web::resource("").route(web::get().to(self::pages::p404)))
    })
    .bind(&dotenv::var("API_URL").unwrap_or(String::from("127.0.0.1:8080")))?
    .run()
    .await
}
