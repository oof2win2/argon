use std::thread;
use std::time::{Duration, Instant};
use indicatif::{ProgressBar, ProgressStyle};
use duration_string::DurationString;
use rusqlite::{Connection};
use crate::backend;
use crate::database;

pub async fn synchronize(dbConnection: &Connection) {
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
		let db_services = database::fetch_services(&dbConnection).unwrap();

		// check if services are existent, if not, create them
		for service in &fetched_services {
			let found = &db_services.iter().find(|s| s.id == service.id);
			if found.is_none() {
				database::create_service(dbConnection, service.secret.clone())
			} else {
				let existing = found.unwrap();
				if existing.secret != service.secret {
					
				}
			}
		}

		let d: String = DurationString::from(start.elapsed()).into();
		pb.finish_with_message(
			"Services have been synchronized in {duration} seconds"
				.replace("{duration}", &d)
		);
	}
}