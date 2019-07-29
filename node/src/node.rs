use redistribution::Blockchain;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};
use std::sync::{Arc, Mutex};
static ROOT_NODE: &str = "127.0.0.1:7878";

use crate::decoder::{DecodedType, Decoder};
use crate::encoder::Encoder;
use crate::peerlist;

use crate::protocol_message::ProtocolMessage;
use peerlist::PeerList;

pub struct Message {
    pub broadcast: bool,
    pub raw_message: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: u128,
    blockchain: Blockchain,
    peerlist: PeerList, // list of IDs
}

impl Node {
    pub fn new() -> Result<Arc<Mutex<Node>>> {
        Ok(Arc::new(Mutex::new(Node {
            id: 0,
            blockchain: Blockchain::new()?,
            peerlist: PeerList::new(),
        })))
    }

    pub fn add_me(&mut self) -> Result<Vec<u8>> {
        let message = Encoder::encode(ProtocolMessage::AddMe, self.id, &String::new())?;
        Ok(message)
    }

    pub fn get_peers(&mut self) -> Result<Vec<u8>> {
        let message = Encoder::encode(ProtocolMessage::GetPeers, self.id, &String::new())?;
        Ok(message)
    }

    pub fn send_transactions(&self) -> Result<Vec<u8>> {
        let transaction = String::from("hello"); // TODO: this should be actual data!
        let message = Encoder::encode(ProtocolMessage::AddTransaction, self.id, &transaction)?;
        Ok(message)
    }

    pub fn get_chain(&mut self) -> Result<Vec<u8>> {
        let message = Encoder::encode(ProtocolMessage::GetBlocks, self.id, &String::new())?;
        Ok(message)
    }

    pub fn handle_message(&mut self, message: &mut Vec<u8>) -> Result<Message> {
        if message.len() == 0 {
            return Err(Error::new(
                ErrorKind::ConnectionAborted,
                "Received 0 bytes message... ignoring",
            ));
        }
        let opcode = Decoder::protocol(&mut message[..]); // TODO: Make buffer as long as headers - then allocate dynamic array buffer for the data

        match opcode {
            Ok(ProtocolMessage::AddMe) => {
                // TODO: ensure we're using UUID. Here we just use an incrementing ID - ideally in the future one node won't store *all* other nodes in its peers... so we'll need a smarter system
                // TODO: This should be the peer node address that we are tracking - how should be handle this?
                let node_addr = ROOT_NODE.parse().unwrap();
                let mut highest_id: u128 = 1;
                let mut peers = self.peerlist.peers.iter();

                while let Some((peer_id, _)) = peers.next() {
                    if highest_id < *peer_id {
                        highest_id = *peer_id;
                    }
                }

                highest_id = highest_id + 1;
                self.peerlist.peers.insert(highest_id, node_addr);

                let message = Encoder::encode(ProtocolMessage::AddedPeer, self.id, &highest_id)?;
                Ok(Message {
                    broadcast: true,
                    raw_message: Some(message)
                })
            }
            Ok(ProtocolMessage::AddedPeer) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::AddedPeer);
                let decoder_type = decoder.decode_json()?;
                match decoder_type {
                    DecodedType::NodeID(node_id) => {
                        self.id = node_id;
                        Ok(Message {
                            broadcast: false,
                            raw_message: None
                        })
                    }
                    _ => Err(Error::new(
                        ErrorKind::Other,
                        "Wrong type passed from decoder",
                    )),
                }
            }
            Ok(ProtocolMessage::GetPeers) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::GetPeers);
                let peer = decoder.peer_id();
                // if self.peerlist.peers.contains_key(&peer) {
                let message = Encoder::encode(ProtocolMessage::PeerList, self.id, &self.peerlist)?;
                Ok(Message {
                    broadcast: false,
                    raw_message: Some(message)
                })
                // } else { // TODO: Handle unrecognised peer again
                //     Err(Error::new(ErrorKind::InvalidData, "Message from unrecognised peer"))
                // }
            }
            Ok(ProtocolMessage::PeerList) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::PeerList);

                let peers = decoder.decode_json()?;
                match peers {
                    DecodedType::PeerList(peerlist) => {
                        println!("Received peers: {:?}", peerlist);
                        self.peerlist = peerlist;
                        Ok(Message {
                            broadcast: false,
                            raw_message: None
                        })
                    },
                    _ => Err(Error::new(ErrorKind::Other, "Did not decode PeerList")),
                }
            }
            Ok(ProtocolMessage::GetBlocks) => {
                let message =
                    Encoder::encode(ProtocolMessage::SendBlockchain, self.id, &self.blockchain)?;
                Ok(Message {
                    broadcast: false,
                    raw_message: Some(message)
                })
            }
            Ok(ProtocolMessage::SendBlockchain) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::SendBlockchain);
                let decoded = decoder.decode_json()?;
                match decoded {
                    DecodedType::Blockchain(blockchain) => {
                        if Blockchain::is_chain_valid(&blockchain) {
                            println!(
                                "Received new chain: {:?}",
                                blockchain
                            );
                            self.blockchain = blockchain;
                            Ok(Message {
                                broadcast: false,
                                raw_message: None
                            })
                        } else {
                            Err(Error::new(
                                ErrorKind::InvalidData, 
                                "Recieved invalid chain"
                            ))
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::InvalidData,
                        "Wrong decoding type used in SendBlockchain command",
                    )),
                }
            }
            Ok(ProtocolMessage::AddTransaction) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::AddTransaction);

                let decoded_type = decoder.decode_json()?;
                match decoded_type {
                    DecodedType::BlockData(data) => {
                        let new_block = self.blockchain.generate_next_block(&data)?;

                        let message =
                            Encoder::encode(ProtocolMessage::NewBlock, self.id, &new_block)?;

                        Ok(Message {
                            broadcast: true,
                            raw_message: Some(message)
                        })
                    }
                    _ => Err(Error::new(
                        ErrorKind::InvalidData,
                        "Wrong decoding type used in AddTransaction command",
                    )),
                }
            }
            Ok(ProtocolMessage::NewBlock) => {
                Ok(Message {
                    broadcast: false,
                    raw_message: None
                })
            }
            Err(e) => Err(e),
            Ok(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "No path for given protocol",
            )),
        }
    }
}
