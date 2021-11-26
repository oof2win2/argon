use rusqlite::{Connection, params};
use crate::models::{Service, NewService};

pub fn init_db(connection: &Connection) {
	connection.execute(
		"CREATE TABLE IF NOT EXISTS `services` (
			id		INTEGER PRIMARY KEY AUTOINCREMENT,
			secret	TEXT NOT NULL
		)", []).ok();
}

pub fn createService(connection: &Connection, secret: Vec<u8>) {
	connection.execute("INSERT INTO `services` (secret) VALUES (?1)", params![secret]).ok();
}

pub fn fetchService(connection: &Connection, id: &i32) -> Option<Service> {
	return connection.query_row("SELECT * FROM `services` WHERE `id` = (?) LIMIT 1", [id], |row| {
		Ok(Service {
			id: row.get(0)?,
			secret: row.get(1)?,
		})
	}).ok();
}

pub fn removeService(connection: &Connection, id: &i32) -> bool {
	let dat = connection.query_row("DELETE * FROM `services` WHERE `id` = (?) LIMIT 1", [id], |row| {
		Ok(true)
	}).ok();
	if dat.is_some() {
		return true
	} else {
		return false
	}
}