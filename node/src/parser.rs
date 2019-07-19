use crate::protocol_message::{ProtocolMessage, Encoding};
use redistribution::{BlockData};
use std::convert::TryFrom;

#[derive(Debug)]
pub enum ParserError {
    UnknownProtocol,
    InvalidOpCode
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
pub struct Parser { 
    raw_bytes: [u8; 512],
    protocol: ProtocolMessage
}

/// Assumes messages apply to format
/// 4 bytes - opcode
/// 16 bytes - peer_id TODO: might want to also include keysize
/// 4 bytes - Message length
/// ... Data
/// 
/// Uses Big Endian
impl Parser {
    pub fn new(raw_bytes: [u8; 512], protocol: ProtocolMessage) -> Parser {
        Parser {
            raw_bytes,
            protocol
        }
    }

    pub fn build_message<'a>(opcode: ProtocolMessage, peer_id: &'a Vec<u8>, message: &'a Vec<u8>) -> Vec<u8> {
        let mut newer = vec!();
        opcode.as_bytes().iter().for_each(|x|{newer.push(*x)});
        peer_id.iter().for_each(|x|{newer.push(*x)});
        let message_length: u128 = u128::try_from(message.len()).unwrap(); // TODO: Handle and return error here - the message is too large - need to come up with better encoding scheme...
        println!("Message length: {}", message_length);
        message_length.to_be_bytes().iter().for_each(|x|{newer.push(*x)}); // puts u8; 16 on message
        message.iter().for_each(|x|{newer.push(*x)});
        Encoding::EndMessage.as_bytes().iter().for_each(|x|{newer.push(*x)}); // TODO: Not sure this is necessary now
        newer
    }

    pub fn opcode(raw_bytes: &mut [u8]) -> Result<ProtocolMessage, ParserError> {
        let mut opcode = [0; 4];
        opcode.swap_with_slice(&mut raw_bytes[Headers::ProtocolType as usize..Headers::PeerEncoding as usize]);
        if opcode == ProtocolMessage::GetBlocks.as_bytes() {
            return Ok(ProtocolMessage::GetBlocks);
        } else if opcode == ProtocolMessage::AddMe.as_bytes() {
            return Ok(ProtocolMessage::AddMe);
        } else if opcode == ProtocolMessage::GetPeers.as_bytes() {
            return Ok(ProtocolMessage::GetPeers);
        } else if opcode == ProtocolMessage::MineBlock.as_bytes() {
            return Ok(ProtocolMessage::MineBlock);
        }
       Err(ParserError::UnknownProtocol)
    }

    /// Reads raw data passed to parser
    /// Ignores first 4 bytes (opcode)
    /// Returns next 16 bytes as u128 - peer ID - removes these from parser
    pub fn peer_id(&mut self) -> u128 {
        let mut bytes_id = [0; 16];
        bytes_id.swap_with_slice(&mut self.raw_bytes[Headers::PeerEncoding as usize..Headers::MessageLength  as usize]);
        u128::from_be_bytes(bytes_id)
    }

    /// Reads raw data passed to parser
    /// Ignores first 4 bytes (opcode)
    /// Ignores next 16 bytes (peer ID)
    /// Parses remainer as blockdata and returns string
    // TODO: what about when the data is bigger than the buffer? How to refactor this?
    pub fn blockdata(&mut self) -> Result<BlockData, ParserError> {
        if self.protocol == ProtocolMessage::MineBlock {
            println!("Received Transaction Data: {:?}", &self.raw_bytes[..]);
            let mut message_length = [0; 16]; // TODO: is this length long enough? What to do when the message runs over?
            message_length.swap_with_slice(&mut self.raw_bytes[Headers::MessageLength as usize..Headers::Data as usize]);
            let length = u128::from_be_bytes(message_length);
            println!("Message: {:?} Length!: {}", message_length, length);
            let index = usize::try_from(length).unwrap(); // TODO: Handle error properly again here.
            let data = self.raw_bytes[Headers::Data as usize..Headers::Data as usize + index].to_vec();

            let json_str = String::from_utf8(data).unwrap();
            println!("Found string: {:?}", json_str);
            let deserialised_data: String = serde_json::from_str(&json_str).unwrap();
            Ok(deserialised_data)
        } else {
            Err(ParserError::InvalidOpCode)
        }
    }
}