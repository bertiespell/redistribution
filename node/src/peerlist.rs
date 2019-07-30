use redistribution::{Decodable, Encodable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerList {
    pub peers: HashMap<uuid::Uuid, SocketAddr>,
}

impl PeerList {
    pub fn new() -> PeerList {
        PeerList {
            peers: HashMap::new(),
        }
    }

    pub fn get_new_peer_id(peer_addr: &[u8]) -> uuid::Uuid {
        Uuid::new_v5(&Uuid::NAMESPACE_OID, peer_addr)
    }
}

impl Encodable for PeerList {
    fn encode(&self) -> Result<Vec<u8>> {
        let peers = serde_json::to_string(&self.peers)?;
        Ok(peers.into_bytes())
    }
}

impl Decodable for PeerList {
    fn decode(bytes: &Vec<u8>) -> Result<Self> {
        let decoded_json_result = String::from_utf8(bytes.clone());
        match decoded_json_result {
            Ok(decoded_json) => {
                let peers: HashMap<uuid::Uuid, SocketAddr> = serde_json::from_str(&decoded_json)?;
                Ok(PeerList { peers })
            }
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode Peerlist - invalid utf8",
            )),
        }
    }
}
