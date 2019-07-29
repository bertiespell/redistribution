use std::io::{Result, Error, ErrorKind};

#[derive(Clone, PartialEq, Debug)]
pub enum ProtocolMessage {
    AddMe, 
	AddedPeer,
    GetPeers,
    NewBlock,
    GetBlocks,
	PeerList,
	AddTransaction,
	SendBlockchain
}

impl ProtocolMessage {
	// get the string value of this message.
	pub fn as_bytes(self) -> &'static [u8] {
		match self {
			ProtocolMessage::AddMe => "0x01".as_bytes(),
			ProtocolMessage::GetPeers => "0x02".as_bytes(),
			ProtocolMessage::NewBlock => "0x03".as_bytes(),
			ProtocolMessage::GetBlocks => "0x04".as_bytes(),
			ProtocolMessage::AddedPeer => "0x05".as_bytes(),
			ProtocolMessage::PeerList => "0x06".as_bytes(),
			ProtocolMessage::AddTransaction => "0x07".as_bytes(),
			ProtocolMessage::SendBlockchain => "0x08".as_bytes(),
		}
	}

	pub fn from_bytes(raw_bytes: &mut [u8]) -> Result<Self> {
		let mut opcode = [0; 4];
		if raw_bytes.len() != 4 {
			return Err(Error::new(ErrorKind::Other, "Cannot construct Protocol opcode from incorrectly sized bytes"));
		}
        opcode.swap_with_slice(raw_bytes);
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
        } else if opcode == ProtocolMessage::SendBlockchain.as_bytes() {
            return Ok(ProtocolMessage::SendBlockchain);
        }
       Err(Error::new(ErrorKind::Other, "Unknown Protocol"))
	}
}