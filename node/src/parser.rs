use serde::{Serialize};
use crate::protocol_message::ProtocolMessage;

pub struct Parser { 
    raw_bytes: [u8; 512]
}

/// Assumes messages apply to format
/// 4 bytes - opcode
/// 16 bytes - peer_id
/// ...
impl Parser {
    pub fn new(raw_bytes: [u8; 512]) -> Parser {
        Parser {
            raw_bytes
        }
    }

    pub fn build_message<T: ?Sized>(opcode: ProtocolMessage, message: &T) -> Vec<u8>
    where
        T: Serialize
    {
        let mut op_code = vec![opcode.as_str()];
        let serialised_value = serde_json::to_string(message).unwrap();
        op_code.push(&serialised_value);
        
        op_code
            .into_iter()
            .map(|astring| astring.as_bytes().to_owned())
            .flatten()
            .collect::<Vec<_>>()
    }

    pub fn opcode(&mut self) -> ProtocolMessage {
        let mut opcode = [0; 4];
        opcode.swap_with_slice(&mut self.raw_bytes[..4]);
        if opcode == ProtocolMessage::GetBlocks.as_bytes() {
            return ProtocolMessage::GetBlocks;
        } else if opcode == ProtocolMessage::AddMe.as_bytes() {
            return ProtocolMessage::AddMe;
        } else if opcode == ProtocolMessage::GetPeers.as_bytes() {
            return ProtocolMessage::GetPeers;
        } else if opcode == ProtocolMessage::MintBlock.as_bytes() {
            return ProtocolMessage::MintBlock;
        }
        panic!()
    }

    pub fn peer_id(&mut self) -> u128 {
        let mut bytes_id = [0; 16];
        bytes_id.swap_with_slice(&mut self.raw_bytes[4..20]);
        u128::from_le_bytes(bytes_id)
    }
}