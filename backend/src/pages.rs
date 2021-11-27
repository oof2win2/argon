use crate::database::{fetchService, fetchServices};
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse};
use rusqlite::Connection;

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

pub async fn getServices(
    pool: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    let servicesResp = fetchServices(&conn);
    if servicesResp.is_ok() {
        HttpResponse::Ok().json(json!({
            "services": servicesResp.unwrap(),
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 500,
            "error": "Server Error",
            "message": "A server error occured",
        }))
    }
}

pub async fn getService(
	pool: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
	id: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
	let conn = pool.get().unwrap();
	let service = fetchService(&conn, &id.to_string().parse::<i32>().unwrap());
	if service.is_some() {
		Ok(HttpResponse::Ok().json(json!({
			"service": service.unwrap()
		})))
	} else {
		Ok(HttpResponse::Ok().json(json!({
			"service": null,
		})))
	}
}