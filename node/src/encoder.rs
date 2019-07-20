use crate::protocol_message::{ProtocolMessage, Encoding};
use redistribution::{BlockData};
use std::convert::TryFrom;
use redistribution::{Encodable, Decodable};

pub type EncodedMessage = Vec<u8>;

/// Stores the first index of each header. Used to break up raw message into relevant sections.
pub enum Headers {
    ProtocolType = 0,
    PeerEncoding = 4,
    MessageLength = 20,
    Data = 36
}

pub struct Encoder {}

impl Encoder {
    // TODO: Handle errors properly!
    fn encode_raw(protocol: ProtocolMessage, peer_id: u128, data: Vec<u8>) -> EncodedMessage {
        let mut raw_encoded = vec!();
        protocol.as_bytes().iter().for_each(|x|{raw_encoded.push(*x)});
        peer_id.to_be_bytes().iter().for_each(|x|{raw_encoded.push(*x)});
        let message_length: u128 = u128::try_from(data.len()).unwrap(); 
        message_length.to_be_bytes().iter().for_each(|x|{raw_encoded.push(*x)});
        data.iter().for_each(|x|{raw_encoded.push(*x)});
        Encoding::EndMessage.as_bytes().iter().for_each(|x|{raw_encoded.push(*x)}); // TODO: Not sure this is necessary now
        raw_encoded
    }

    pub fn encode<T: Encodable>(protocol: ProtocolMessage, peer_id: u128, data: &T) -> EncodedMessage {
        Encoder::encode_raw(protocol, peer_id, data.encode().to_vec())
    }
}