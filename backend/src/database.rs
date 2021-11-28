use rusqlite::{params, Connection};
use crate::functions::{decode, encode};
use crate::models::{Service, CreateService};

pub fn init_db(connection: &Connection) {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS `services` (
				id		INTEGER PRIMARY KEY AUTOINCREMENT,
				name	TEXT NOT NULL,
				color	TEXT NOT NULL,
				secret	TEXT NOT NULL
			)",
            [],
        )
        .ok();
}

pub fn create_service(connection: &Connection, service: &CreateService) -> Option<Service> {
    connection
        .execute(
            "INSERT INTO `services` (name, color, secret) VALUES (?1, ?2, ?3)",
            [&service.name, &service.color, &service.secret],
        )
        .ok();
	
	let result = fetch_service(connection, &(connection.last_insert_rowid() as u32));
	if result.is_some() {
		let created = result.unwrap();
		if 	created.color == service.color &&
			created.name == service.name && 
			created.secret == decode(&service.secret) {
			return Some(created);
		} else {
			return None;
		}
	} else {
		return None;
	}

}

pub fn fetch_service(connection: &Connection, id: &u32) -> Option<Service> {
    return connection
        .query_row(
            "SELECT * FROM `services` WHERE `id` = (?) LIMIT 1",
            [id],
            |row| {
                Ok(Service {
					id: row.get::<_, u32>(0)?,
					name: row.get::<_, String>(1)?,
					color: row.get::<_, String>(2)?,
					secret: decode(&row.get::<_, String>(3)?),
                })
            },
        )
        .ok();
}

pub fn fetch_services(connection: &Connection) -> Result<Vec<Service>, rusqlite::Error> {
    let mut stmt = connection.prepare("SELECT * FROM `services`")?;
    let mut rows = stmt.query([])?;

    let mut data = Vec::new();
    while let Some(row) = rows.next()? {
        data.push(Service {
            id: row.get::<_, u32>(0)?,
			name: row.get::<_, String>(1)?,
			color: row.get::<_, String>(2)?,
            secret: decode(&row.get::<_, String>(3)?),
        })
    }
    Ok(data)
}

pub fn remove_service(connection: &Connection, id: &i32) -> bool {
    let dat = connection
        .execute(
            "DELETE FROM `services` WHERE `id` = (?) LIMIT 1",
            [id]
        )
        .ok();
    if dat.is_some() {
		let affected = dat.unwrap();
		return if affected == 1 {true} else {false};
    } else {
        return false;
    }
}
