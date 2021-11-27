use crate::functions::encode;
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};
use serde::ser::SerializeStruct;

pub struct Service {
    pub id: u32,
    pub secret: Vec<u8>,
}

impl Serialize for Service {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut service = serializer.serialize_struct("Service", 16)?;
        service.serialize_field("id", &self.id)?;
        service.serialize_field("secret", &encode(&self.secret))?;
        return service.end()
    }
}

pub struct NewService<'a> {
    pub secret: &'a str,
}
