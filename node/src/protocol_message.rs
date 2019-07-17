pub enum ProtocolMessage {
    AddMe, 
    GetPeers,
    MintBlock,
    GetBlocks,
    PeerAdded
}

impl ProtocolMessage {
	// get the string value of this message.
	pub fn as_str(self) -> &'static str {
		match self {
			ProtocolMessage::AddMe => "0x00",
			ProtocolMessage::GetPeers => "0x01",
			ProtocolMessage::MintBlock => "0x02",
			ProtocolMessage::GetBlocks => "0x03",
            ProtocolMessage::PeerAdded => "0x04"
		}
	}

	// get the string value of this message.
	pub fn as_bytes(self) -> &'static [u8] {
		match self {
			ProtocolMessage::AddMe => "0x00".as_bytes(),
			ProtocolMessage::GetPeers => "0x01".as_bytes(),
			ProtocolMessage::MintBlock => "0x02".as_bytes(),
			ProtocolMessage::GetBlocks => "0x03".as_bytes(),
            ProtocolMessage::PeerAdded => "0x04".as_bytes()
		}
	}

	// try to parse the message value from a string.
	pub fn from_str(s: &str) -> Option<Self> {
		match s {
			"0x00" => Some(ProtocolMessage::AddMe),
			"0x01" => Some(ProtocolMessage::GetPeers),
			"0x02" => Some(ProtocolMessage::MintBlock),
			"0x03" => Some(ProtocolMessage::GetBlocks),
            "0x04" => Some(ProtocolMessage::PeerAdded),
			_ => None
		}
	}
}