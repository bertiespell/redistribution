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
}