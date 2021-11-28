use crate::functions::encode;
use serde::ser::SerializeStruct;
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize};

pub struct Service {
    pub id: u32,
	pub name: String,
	pub color: String,
    pub secret: Vec<u8>,
}

impl Serialize for Service {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut service = serializer.serialize_struct("Service", 16)?;
        service.serialize_field("id", &self.id)?;
        service.serialize_field("name", &self.name)?;
        service.serialize_field("color", &self.color)?;
        service.serialize_field("secret", &encode(&self.secret))?;
        return service.end();
    }
}

#[derive(Deserialize)]
pub struct CreateService {
	pub name: String,
	pub color: String,
    pub secret: String,
}
