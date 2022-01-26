use crate::models::{Service};
use rusqlite::{params, Connection};

pub fn init_db(connection: &Connection) {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS `services` (
			id		INTEGER PRIMARY KEY AUTOINCREMENT,
			name 	TEXT NOT NULL,
			secret	TEXT NOT NULL
		)",
            [],
        )
        .ok();
}

pub fn create_service(connection: &Connection, service: &crate::models::Service) {
    connection
        .execute(
            "INSERT INTO `services` (id, name, secret) VALUES (?1, ?2, ?3)",
            params![service.id, service.name, service.secret],
        ).ok();
}

pub fn fetch_service(connection: &Connection, id: &i32) -> Option<Service> {
    return connection
        .query_row(
            "SELECT * FROM `services` WHERE `id` = (?) LIMIT 1",
            [id],
            |row| {
                Ok(Service {
                    id: row.get(0)?,
					name: row.get(1)?,
                    secret: row.get(2)?,
                })
            },
        )
        .ok();
}

pub fn update_service(connection: &Connection, service: &Service) {
	connection
		.execute(
			"UPDATE `services` SET name = ?1, secret = ?2 WHERE id = ?3",
			params![service.name, service.secret, service.id],
		).ok();
}

pub fn remove_service(connection: &Connection, id: &i32) -> bool {
	let dat = connection
		.query_row(
			"DELETE * FROM `services` WHERE `id` = (?) LIMIT 1",
			[id],
			|_row| Ok(true),
		)
		.ok();
	if dat.is_some() {
		return true;
	} else {
		return false;
	}
}

pub fn fetch_services(connection: &Connection) -> Result<Vec<Service>, rusqlite::Error> {
	let mut stmt = connection.prepare("SELECT * FROM `services`")?;
    let mut rows = stmt.query([])?;

    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        data.push(Service {
            id: row.get::<_, u32>(0)?,
			name: row.get::<_, String>(1)?,
            secret: row.get::<_, Vec<u8>>(2)?,
        })
    }
	Ok(data)
}

