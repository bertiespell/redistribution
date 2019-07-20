use crate::protocol_message::{ProtocolMessage, Encoding};
use redistribution::{BlockData};
use std::convert::TryFrom;
use redistribution::{Encodable, Decodable};
use crate::node;
use node::PeerList;

#[derive(Debug)]
pub enum DecoderError {
    UnknownProtocol,
    InvalidOpCode,
    NoDecodeAvailable
}

/// Stores the first index of each header. Used to break up raw message into relevant sections.
pub enum Headers {
    ProtocolType = 0,
    PeerEncoding = 4,
    MessageLength = 20,
    Data = 36
}

/// Handles reading/writing encoding structure
/// Must be passed the protocol to avoid incorrect parsing
/// Uses the following encoding schema
///     First 4 bytes: Opcode
///     Second 16 bytes: Opcode
pub struct Decoder { 
    raw_bytes: [u8; 512],
    protocol: ProtocolMessage
}

pub enum DecodedType {
    BlockData(BlockData),
    Node_ID(u128),
    PeerList(PeerList),
}

/// Assumes messages apply to format
/// 4 bytes - opcode
/// 16 bytes - peer_id TODO: might want to also include keysize
/// 4 bytes - Message length
/// ... Data
/// 
/// Uses Big Endian
impl Decoder {
    pub fn new(raw_bytes: [u8; 512], protocol: ProtocolMessage) -> Decoder {
        Decoder {
            raw_bytes,
            protocol
        }
    }

    pub fn opcode(raw_bytes: &mut [u8]) -> Result<ProtocolMessage, DecoderError> {
        let mut opcode = [0; 4];
        opcode.swap_with_slice(&mut raw_bytes[Headers::ProtocolType as usize..Headers::PeerEncoding as usize]);
        if opcode == ProtocolMessage::GetBlocks.as_bytes() {
            return Ok(ProtocolMessage::GetBlocks);
        } else if opcode == ProtocolMessage::AddMe.as_bytes() {
            return Ok(ProtocolMessage::AddMe);
        } else if opcode == ProtocolMessage::GetPeers.as_bytes() {
            return Ok(ProtocolMessage::GetPeers);
        } else if opcode == ProtocolMessage::NewBlock.as_bytes() {
            return Ok(ProtocolMessage::NewBlock);
        } else if opcode == ProtocolMessage::AddTransaction.as_bytes() {
            return Ok(ProtocolMessage::AddTransaction);
        } else if opcode == ProtocolMessage::AddedPeer.as_bytes() {
            return Ok(ProtocolMessage::AddedPeer);
        }
       Err(DecoderError::UnknownProtocol)
    }

    /// Reads raw data passed to parser
    /// Ignores first 4 bytes (opcode)
    /// Returns next 16 bytes as u128 - peer ID - removes these from parser
    pub fn peer_id(&mut self) -> u128 {
        let mut bytes_id = [0; 16];
        bytes_id.swap_with_slice(&mut self.raw_bytes[Headers::PeerEncoding as usize..Headers::MessageLength  as usize]);
        u128::from_be_bytes(bytes_id)
    }

    pub fn message_length(&mut self) -> u128 {
        let mut bytes_id = [0; 16];
        bytes_id.swap_with_slice(&mut self.raw_bytes[Headers::MessageLength as usize..Headers::Data as usize]);
        u128::from_be_bytes(bytes_id)
    }

    /// Reads raw data passed to parser
    /// Ignores first 4 bytes (opcode)
    /// Ignores next 16 bytes (peer ID)
    /// Parses remainer as blockdata and returns string
    // TODO: what about when the data is bigger than the 512 buffer? How to refactor this?
    pub fn decode_raw(&mut self) -> Result<Vec<u8>, DecoderError> {
        let index = usize::try_from(self.message_length()).unwrap(); // TODO: Handle error properly again here.
        Ok(self.raw_bytes[Headers::Data as usize..Headers::Data as usize + index].to_vec())
    }

    pub fn decode_json(&mut self) -> Result<DecodedType, DecoderError> {
        match self.protocol {
            ProtocolMessage::AddTransaction => {
                let decoded_data = self.decode_raw().unwrap();
                let json_str = String::from_utf8(decoded_data).unwrap();
                let deserialised_data: BlockData = serde_json::from_str(&json_str).unwrap();
                Ok(DecodedType::BlockData(deserialised_data))
            },
            ProtocolMessage::AddedPeer => {
                let decoded = u128::decode(&self.decode_raw().unwrap());
                Ok(DecodedType::Node_ID(decoded))
            },
            ProtocolMessage::PeerList => {
                let raw_data = self.decode_raw().unwrap();
                let peerlist = PeerList::decode(&raw_data);
                Ok(DecodedType::PeerList(peerlist))
            }
            _ => Err(DecoderError::NoDecodeAvailable)
        }
    }
}