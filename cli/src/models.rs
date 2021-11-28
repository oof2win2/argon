use serde::Deserialize;

pub struct Service {
    pub id: u32,
    pub secret: Vec<u8>,
}

#[derive(Deserialize)]
pub struct TransportService {
    pub id: u32,
    pub secret: String,
}

pub struct NewService<'a> {
    pub secret: &'a str,
}
