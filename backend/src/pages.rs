use actix_web::{web, App, HttpResponse};
use actix_web::http::{StatusCode};
use crate::database::{fetchService, fetchServices};
use rusqlite::{Connection};

/**
 * 404 page
 */
pub async fn p404() -> HttpResponse {
	HttpResponse::Ok().json(json!({
		"code": 404,
		"error": "Not Found",
		"message": "Page Not Found",
	}))
}

pub async fn getServices(pool: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>) -> HttpResponse {
	let conn = pool.get().unwrap();
	let servicesResp = fetchServices(&conn);
	if servicesResp.is_ok() {
		let services = servicesResp.unwrap();
		let mut dat = Vec::new();
		for service in services {
			dat.push(json!({
				"id": service.id,
				"secret": crate::functions::encode(&service.secret),
			}))
		}
		HttpResponse::Ok().json(json!({
			"services": dat,
		}))
	} else {
		HttpResponse::Ok().json(json!({
			"code": 500,
			"error": "Server Error",
			"message": "A server error occured",
		}))
	}
}