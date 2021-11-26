pub struct Service {
	pub id: i32,
	pub secret: Vec<u8>
}

pub struct NewService<'a> {
    pub secret: &'a str,
}