impl Encodable for u128 {
    fn encode(&self) -> Vec<u8> {
        let serialized = serde_json::to_string(&self).unwrap();
        serialized.into_bytes()
    }
}

impl Decodable for u128 {
    fn decode(&self, bytes: &Vec<u8>) -> Self {
        let json_string = String::from_utf8(bytes.clone()).unwrap();
        let deserialized: u128 = serde_json::from_str(&json_string).unwrap();
        deserialized
    }
}

pub trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decodable: Sized {
    fn decode(&self, bytes: &Vec<u8>) -> Self;
}