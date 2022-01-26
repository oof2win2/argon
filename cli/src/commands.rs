use std::time::{Instant};
use indicatif::{ProgressBar, ProgressStyle};
use duration_string::DurationString;
use rusqlite::{Connection};
use crate::backend;
use crate::database;

pub async fn synchronize(db_connection: &Connection) {
	let start = Instant::now();
	let pb = ProgressBar::new_spinner();
	pb.enable_steady_tick(120);
	pb.set_style(
		ProgressStyle::default_spinner()
	);
	pb.set_message("Fetching services");
	let fetch = backend::get_services().await;
	if fetch.is_ok() {
		let fetched_services = fetch.ok().unwrap();
		let db_services = database::fetch_services(db_connection).unwrap();
		println!("fetched length {}", db_services.len());

		// check if services are existent, if not, create them
		for service in &fetched_services {
			let found = db_services.iter().find(|s| s.id == service.id);
			if found.is_none() {
				database::create_service(db_connection, &service);
			} else {
				let existing = found.unwrap();
				if existing.secret != service.secret || existing.name != service.name {
					// the secrets of the two are not correct, update the db service
					database::update_service(db_connection, &service);
				}
			}
		}

		let d: String = DurationString::from(start.elapsed()).into();
		pb.finish_with_message(
			"Services have been synchronized in {duration}"
				.replace("{duration}", &d)
		);
	} else {
		pb.finish_with_message("Could not synchronize srevices")
	}
}

pub async fn get_service(db_connection: &Connection, id: i32) -> Option<crate::models::Service> {
	let service = database::fetch_service(db_connection, &id);
	if service.is_some() {
		return service;
	}
	return None;
}