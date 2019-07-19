use serde::{Serialize};
use crate::protocol_message::ProtocolMessage;

#[derive(Debug)]
pub enum ParserError {
    UnknownProtocol
}

/// Handles reading/writing encoding structure
/// Must be passed the protocol to avoid incorrect parsing
/// Uses the following encoding schema
///     First 4 bytes: Opcode
///     Second 16 bytes: Opcode
pub struct Parser { 
    raw_bytes: [u8; 512],
    protocol: ProtocolMessage
}

/// Assumes messages apply to format
/// 4 bytes - opcode
/// 16 bytes - peer_id TODO: might want to also include keysize
/// Remaining bytes - data
/// ...
impl Parser {
    pub fn new(raw_bytes: [u8; 512], protocol: ProtocolMessage) -> Parser {
        Parser {
            raw_bytes,
            protocol
        }
    }

    /// Message should be a data object consisting of key-value pairs.
    /// Uses serde
    pub fn build_json_message<T: ?Sized>(opcode: ProtocolMessage, message: &T) -> Vec<u8>
    where
        T: Serialize
    {
        let mut data = vec![opcode.as_str()];
        let serialised_value = serde_json::to_string(message).unwrap();
        data.push(&serialised_value);
        
        data
            .into_iter()
            .map(|astring| astring.as_bytes().to_owned())
            .flatten()
            .collect::<Vec<_>>()
    }

    pub fn build_raw_message<'a>(opcode: ProtocolMessage, message: &'a Vec<u8>) -> Vec<u8> {
        let mut newer = vec!();
        opcode.as_bytes().iter().for_each(|x|{newer.push(*x)});
        message.iter().for_each(|x|{newer.push(*x)});
        newer
    }

    pub fn opcode(raw_bytes: &mut [u8]) -> Result<ProtocolMessage, ParserError> {
        let mut opcode = [0; 4];
        opcode.swap_with_slice(&mut raw_bytes[..4]);
        if opcode == ProtocolMessage::GetBlocks.as_bytes() {
            return Ok(ProtocolMessage::GetBlocks);
        } else if opcode == ProtocolMessage::AddMe.as_bytes() {
            return Ok(ProtocolMessage::AddMe);
        } else if opcode == ProtocolMessage::GetPeers.as_bytes() {
            return Ok(ProtocolMessage::GetPeers);
        } else if opcode == ProtocolMessage::MintBlock.as_bytes() {
            return Ok(ProtocolMessage::MintBlock);
        }
       Err(ParserError::UnknownProtocol)
    }

    /// Reads raw data passed to parser
    /// Ignores first 4 bytes (opcode)
    /// Returns next 16 bytes as u128 - peer ID
    pub fn peer_id(&mut self) -> u128 {
        let mut bytes_id = [0; 16];
        bytes_id.swap_with_slice(&mut self.raw_bytes[4..20]);
        u128::from_be_bytes(bytes_id)
    }
}