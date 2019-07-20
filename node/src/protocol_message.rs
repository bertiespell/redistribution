pub enum Encoding {
	ParseString,
	ParseU128,
	EndMessage,
}

impl Encoding {
	// get the string value of this message.
	pub fn as_str(self) -> &'static str {
		match self {
			Encoding::ParseString => "0x11",
			Encoding::ParseU128 => "0x11",
			Encoding::EndMessage => "0x11",

		}
	}

	// get the string value of this message.
	pub fn as_bytes(self) -> &'static [u8] {
		match self {
			Encoding::ParseString => "0x11".as_bytes(),
			Encoding::ParseU128 => "0x11".as_bytes(),
			Encoding::EndMessage => "0x11".as_bytes(),

		}
	}
}

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
	pub fn as_str(self) -> &'static str {
		match self {
			ProtocolMessage::AddMe => "0x01",
			ProtocolMessage::GetPeers => "0x02",
			ProtocolMessage::NewBlock => "0x03",
			ProtocolMessage::GetBlocks => "0x04",
			ProtocolMessage::AddedPeer => "0x05",
			ProtocolMessage::PeerList => "0x06",
			ProtocolMessage::AddTransaction => "0x07",
			ProtocolMessage::SendBlockchain => "0x08",
		}
	}

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
}