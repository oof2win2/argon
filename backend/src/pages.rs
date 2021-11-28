use crate::database;
use crate::models;
use actix_web::{web, HttpResponse};

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

pub async fn get_services(
    pool: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    let services_resp = database::fetch_services(&conn);
    if services_resp.is_ok() {
        HttpResponse::Ok().json(json!({
            "services": services_resp.unwrap(),
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "code": 500,
            "error": "Server Error",
            "message": "A server error occured",
        }))
    }
}

pub async fn get_service(
    pool: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
    id: web::Path<String>,
) -> HttpResponse {
    let conn = pool.get().unwrap();
    let service = database::fetch_service(&conn, &id.to_string().parse::<u32>().unwrap());
    if service.is_some() {
        HttpResponse::Ok().json(json!({
            "service": service.unwrap()
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "service": null,
        }))
    }
}

pub async fn create_service(
	pool: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
	body: web::Json<models::CreateService>,
) -> HttpResponse {
	let conn = pool.get().unwrap();
    let service = database::create_service(&conn, &body);
    if service.is_some() {
        HttpResponse::Ok().json(json!({
            "service": service.unwrap()
        }))
    } else {
        HttpResponse::Ok().json(json!({
            "service": null,
        }))
    }
}

pub async fn remove_service(
	pool: web::Data<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
    id: web::Path<String>,
) -> HttpResponse {
	let conn = pool.get().unwrap();
    let removed = database::remove_service(&conn, &id.to_string().parse::<i32>().unwrap());
    HttpResponse::Ok().json(json!({
		"removed": removed
	}))
}