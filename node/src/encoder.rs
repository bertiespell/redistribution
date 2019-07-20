use crate::protocol_message::{ProtocolMessage};
use std::convert::TryFrom;
use redistribution::{Encodable};
use std::io::{Result, Error, ErrorKind};

pub type EncodedMessage = Vec<u8>;

pub struct Encoder {}

impl Encoder {
    fn encode_raw(protocol: ProtocolMessage, peer_id: u128, data: Vec<u8>) -> Result<EncodedMessage> {
        let mut raw_encoded = vec!();
        protocol
            .as_bytes()
            .iter()
            .for_each(|x|raw_encoded.push(*x));
        peer_id
            .to_be_bytes()
            .iter()
            .for_each(|x|raw_encoded.push(*x));
        let message_length_result = u128::try_from(data.len());
        match message_length_result {
            Ok(message_length) => {
                message_length
                    .to_be_bytes()
                    .iter()
                    .for_each(|x|raw_encoded.push(*x));
                data
                    .iter()
                    .for_each(|x|raw_encoded.push(*x));
                Encoding::EndMessage
                    .as_bytes()
                    .iter()
                    .for_each(|x|raw_encoded.push(*x));
                Ok(raw_encoded)
            },
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Failed to encode message length as u128 from data length"))
        } 
    }

    pub fn encode<T: Encodable>(protocol: ProtocolMessage, peer_id: u128, data: &T) -> Result<EncodedMessage> {
        Encoder::encode_raw(protocol, peer_id, data.encode().to_vec())
    }
}

pub enum Encoding {
	EndMessage,
}

impl Encoding {
	// get the string value of this message.
	pub fn as_bytes(self) -> &'static [u8] {
		match self {
			Encoding::EndMessage => "0x11".as_bytes(),

		}
	}
}