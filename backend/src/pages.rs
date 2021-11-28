use crate::database::{fetch_service, fetch_services};
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
    let services_resp = fetch_services(&conn);
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
) -> Result<HttpResponse, actix_web::Error> {
    let conn = pool.get().unwrap();
    let service = fetch_service(&conn, &id.to_string().parse::<i32>().unwrap());
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
