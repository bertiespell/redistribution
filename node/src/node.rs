use redistribution::Blockchain;
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::decoder::{DecodedType, Decoder};
use crate::encoder::Encoder;
use crate::peerlist;

use crate::protocol_message::ProtocolMessage;
use peerlist::PeerList;

#[derive(Debug)]
pub struct Message {
    pub broadcast: bool,
    pub connect: Option<SocketAddr>,
    pub raw_message: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    pub id: Uuid,
    blockchain: Blockchain,
    pub peerlist: PeerList,
    address: String,
}

impl Node {
    pub fn new(address: String) -> Arc<Mutex<Node>> {
        Arc::new(Mutex::new(Node {
            id: PeerList::get_new_peer_id(address.as_bytes()),
            blockchain: Blockchain::new(),
            peerlist: PeerList::new(),
            address,
        }))
    }

    pub fn add_me(&mut self) -> Result<Vec<u8>> {
        let message = Encoder::encode(ProtocolMessage::AddMe, self.id, &self.address)?;
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
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::AddMe);
                let decoder_type = decoder.decode_json()?;
                match decoder_type {
                    DecodedType::NewPeer(peer_ip) => {
                        let new_key = self.peerlist.peers.insert(decoder.peer_id(), peer_ip); // TODO: this should return error if peer already exists (ID is taken)
                        match new_key {
                            Some(_) => {
                                // If we already had the key - no need to rebroadcast
                                Ok(Message {
                                    broadcast: false,
                                    connect: None,
                                    raw_message: None,
                                })
                            }
                            None => {
                                println!("Updated peerlist: {:?}", self.peerlist.peers);

                                let message = Encoder::encode(
                                    ProtocolMessage::UpdatePeer,
                                    decoder.peer_id(),
                                    &peer_ip.to_string(),
                                )?;

                                Ok(Message {
                                    broadcast: true,
                                    connect: None,
                                    raw_message: Some(message),
                                })
                            }
                        }
                    }
                    _ => Err(Error::new(
                        ErrorKind::Other,
                        "Wrong type passed from decoder",
                    )),
                }
            }
            Ok(ProtocolMessage::UpdatePeer) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::UpdatePeer);
                let decoder_type = decoder.decode_json()?;

                match decoder_type {
                    DecodedType::UpdatePeer(peer_id, peer_ip) => {
                        let new_key = self.peerlist.peers.insert(peer_id, peer_ip); // TODO: this should return error if peer already exists (ID is taken)
                        match new_key {
                            Some(_) => {
                                // If we already had the key - no need to rebroadcast
                                Ok(Message {
                                    broadcast: false,
                                    connect: None,
                                    raw_message: None,
                                })
                            }
                            None => {
                                println!("Updated peerlist: {:?}", self.peerlist.peers);

                                let message = Encoder::encode(
                                    ProtocolMessage::UpdatePeer,
                                    decoder.peer_id(),
                                    &peer_ip.to_string(),
                                )?;

                                Ok(Message {
                                    broadcast: true,
                                    connect: Some(peer_ip),
                                    raw_message: Some(message),
                                })
                            }
                        }
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
                if self.peerlist.peers.contains_key(&peer) {
                    let message =
                        Encoder::encode(ProtocolMessage::PeerList, self.id, &self.peerlist)?;
                    Ok(Message {
                        broadcast: false,
                        connect: None,
                        raw_message: Some(message),
                    })
                } else {
                    // TODO: Handle unrecognised peer again
                    Err(Error::new(
                        ErrorKind::InvalidData,
                        "Message from unrecognised peer",
                    ))
                }
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
                            connect: None,
                            raw_message: None,
                        })
                    }
                    _ => Err(Error::new(ErrorKind::Other, "Did not decode PeerList")),
                }
            }
            Ok(ProtocolMessage::GetBlocks) => {
                let message =
                    Encoder::encode(ProtocolMessage::SendBlockchain, self.id, &self.blockchain)?;
                Ok(Message {
                    broadcast: false,
                    connect: None,
                    raw_message: Some(message),
                })
            }
            Ok(ProtocolMessage::SendBlockchain) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::SendBlockchain);
                let decoded = decoder.decode_json()?;
                match decoded {
                    DecodedType::Blockchain(blockchain) => {
                        if Blockchain::is_chain_valid(&blockchain) {
                            println!("Received new chain: {:?}", blockchain);
                            self.blockchain = blockchain;
                            Ok(Message {
                                broadcast: false,
                                connect: None,
                                raw_message: None,
                            })
                        } else {
                            Err(Error::new(ErrorKind::InvalidData, "Recieved invalid chain"))
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
                            connect: None,
                            raw_message: Some(message),
                        })
                    }
                    _ => Err(Error::new(
                        ErrorKind::InvalidData,
                        "Wrong decoding type used in AddTransaction command",
                    )),
                }
            }
            Ok(ProtocolMessage::NewBlock) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::NewBlock);

                let decoded_type = decoder.decode_json()?;
                println!("Received new block: {:?}", decoded_type);
                Ok(Message {
                    broadcast: false,
                    connect: None,
                    raw_message: None,
                })
            }
            Err(e) => Err(e),
        }
    }
}
