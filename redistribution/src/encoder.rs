use std::io::{Error, ErrorKind, Result};

impl Encodable for u128 {
    fn encode(&self) -> Result<Vec<u8>> {
        let serialized = serde_json::to_string(&self)?;
        Ok(serialized.into_bytes())
    }
}

impl Decodable for u128 {
    fn decode(bytes: &Vec<u8>) -> Result<Self> {
        let json_string_result = String::from_utf8(bytes.clone());
        match json_string_result {
            Ok(json_string) => {
                let deserialized: u128 = serde_json::from_str(&json_string)?;
                Ok(deserialized)
            }
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to decode u128 - bytes not valid utf8",
            )),
        }
    }
}

impl Encodable for String {
    fn encode(&self) -> Result<Vec<u8>> {
        let serialized = serde_json::to_string(&self)?;
        Ok(serialized.into_bytes())
    }
}

impl Decodable for String {
    fn decode(bytes: &Vec<u8>) -> Result<Self> {
        let json_string_result = String::from_utf8(bytes.clone());
        match json_string_result {
            Ok(json_string) => {
                let deserialized: String = serde_json::from_str(&json_string)?;
                Ok(deserialized)
            }
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to decode String - bytes not valid utf8",
            )),
        }
    }
}

pub trait Encodable {
    fn encode(&self) -> Result<Vec<u8>>;
}

pub trait Decodable: Sized {
    fn decode(bytes: &Vec<u8>) -> Result<Self>;
}
