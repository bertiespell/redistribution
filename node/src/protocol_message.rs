#[derive(Clone)]
pub enum ProtocolMessage {
    AddMe, 
    GetPeers,
    MintBlock,
    GetBlocks,
}

impl ProtocolMessage {
	// get the string value of this message.
	pub fn as_str(self) -> &'static str {
		match self {
			ProtocolMessage::AddMe => "0x00",
			ProtocolMessage::GetPeers => "0x01",
			ProtocolMessage::MintBlock => "0x02",
			ProtocolMessage::GetBlocks => "0x03",
		}
	}

	// get the string value of this message.
	pub fn as_bytes(self) -> &'static [u8] {
		match self {
			ProtocolMessage::AddMe => "0x00".as_bytes(),
			ProtocolMessage::GetPeers => "0x01".as_bytes(),
			ProtocolMessage::MintBlock => "0x02".as_bytes(),
			ProtocolMessage::GetBlocks => "0x03".as_bytes(),
		}
	}
}