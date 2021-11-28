use crate::functions::decode;
use crate::models::{Service, TransportService};
use serde::Deserialize;

#[derive(Deserialize)]
struct GetServicesResponse {
    services: Vec<TransportService>,
}
pub async fn get_services() -> Result<Vec<Service>, Box<dyn std::error::Error>> {
    let resp = reqwest::get("{apiurl}/services".replace(
        "{apiurl}",
        &dotenv::var("BACKEND_URL").unwrap_or(String::from("http://127.0.0.1:8080")),
    ))
    .await?
    .json::<GetServicesResponse>()
    .await?;

    let mut services = Vec::new();
    for service in resp.services {
        services.push(Service {
            id: service.id,
            secret: decode(&service.secret).unwrap(),
        })
    }
    Ok(services)
}

#[derive(Deserialize)]
struct GetServiceResponse {
    service: Option<TransportService>,
}
pub async fn get_service(id: &i32) -> Result<Option<Service>, Box<dyn std::error::Error>> {
    println!(
        "{}/services/{}",
        &dotenv::var("BACKEND_URL").unwrap_or(String::from("http://127.0.0.1:8080")),
        &id.to_string()
    );
    let resp = reqwest::get(
        "{apiurl}/services/{id}"
            .replace(
                "{apiurl}",
                &dotenv::var("BACKEND_URL").unwrap_or(String::from("http://127.0.0.1:8080")),
            )
            .replace("{id}", &id.to_string()),
    )
    .await?
    .json::<GetServiceResponse>()
    .await?;

    if resp.service.is_some() {
        let s = resp.service.unwrap();
        let service = Service {
            id: s.id,
            secret: decode(&s.secret).unwrap(),
        };
        return Ok(Some(service));
    } else {
        return Ok(None);
    }
}
