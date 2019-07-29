use std::io::prelude::*;
use std::io::{Result, Error, ErrorKind};
use std::net::{TcpStream, Shutdown};
use redistribution::{Blockchain};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use std::net::{SocketAddr};
static ROOT_NODE: &str = "127.0.0.1:7878";

use crate::encoder::{Encoder};
use crate::decoder::{Decoder, DecodedType};
use crate::peerlist;

use crate::protocol_message::ProtocolMessage;
use peerlist::PeerList;

pub struct Message {
    broadcast: bool,
    message: Option<Vec<u8>>
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

    pub fn add_me(&mut self, stream: &mut TcpStream) -> Result<()> {
        let message = Encoder::encode(ProtocolMessage::AddMe, self.id, &String::new())?;

        stream.write(&message)?;
        let mut buffer = [0; 512];
        let result = stream.read(&mut buffer);
        match result {
            Ok(_) => {
                // TODO: pull the node ID out of the encoder
                let mut decoder = Decoder::new(&mut buffer, ProtocolMessage::AddedPeer);
                let decoder_type = decoder.decode_json();
                match decoder_type {
                    Ok(DecodedType::NodeID(node_id)) => {
                        self.id = node_id;
                        Ok(())
                    },
                    Err(_) => {
                        Err(Error::new(ErrorKind::Other, "Error decoding Node ID"))
                    },
                    _ => {
                        Err(Error::new(ErrorKind::Other, "Wrong type passed from decoder"))
                    }
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_peers(&mut self, stream: &mut TcpStream) -> Result<()> {
        let message = Encoder::encode(ProtocolMessage::GetPeers, self.id, &String::new())?;
        let mut buffer = [0; 512];
        stream.write(&message[..])?;
        let result = stream.read(&mut buffer);

        match result {
            Ok(_) => {            
                let mut decoder = Decoder::new(&mut buffer, ProtocolMessage::PeerList);

                let peers = decoder.decode_json();
                match peers {
                    Ok(DecodedType::PeerList(peerlist)) => {
                        self.peerlist = peerlist;
                        Ok(())
                    },
                    _ => Err(Error::new(ErrorKind::Other, "Did not decode PeerList")) // TODO: handle erros properly... again! (Handle error two error cases here)
                }
                
            },
            Err(e) => Err(e),
        }
    }

    pub fn send_transactions(&self) -> Result<Vec<u8>> {
        let transaction = String::from("hello"); // TODO: this should be actual data!
        let message = Encoder::encode(ProtocolMessage::AddTransaction, self.id, &transaction)?;
        Ok(message)
    }

    pub fn get_chain(&mut self) -> Result<Vec<u8>>  {
        let message = Encoder::encode(ProtocolMessage::GetBlocks, self.id, &String::new())?;
        Ok(message)
    }

    pub fn handle(&mut self, message: &mut Vec<u8>) -> Result<Vec<u8>> {
        if message.len() == 0 {
            return Err(Error::new(ErrorKind::ConnectionAborted, "Received 0 bytes message... ignoring"));
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
                Ok(message)
            },
            Ok(ProtocolMessage::AddedPeer) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::AddedPeer);
                let decoder_type = decoder.decode_json();
                match decoder_type {
                    Ok(DecodedType::NodeID(node_id)) => {
                        self.id = node_id;
                        Ok(vec!())
                    },
                    Err(_) => {
                        Err(Error::new(ErrorKind::Other, "Error decoding Node ID"))
                    },
                    _ => {
                        Err(Error::new(ErrorKind::Other, "Wrong type passed from decoder"))
                    }
                }
            },
            Ok(ProtocolMessage::GetPeers) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::GetPeers);
                let peer = decoder.peer_id();
                if self.peerlist.peers.contains_key(&peer) {
                    let message = Encoder::encode(ProtocolMessage::PeerList, self.id, &self.peerlist)?;
                    Ok(message)
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "Message from unrecognised peer"))
                }
            },
            Ok(ProtocolMessage::PeerList) => {
                println!("Peerlist received");
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::PeerList);

                let peers = decoder.decode_json();
                match peers {
                    Ok(DecodedType::PeerList(peerlist)) => {
                        self.peerlist = peerlist;
                        Ok(vec!())
                    },
                    _ => Err(Error::new(ErrorKind::Other, "Did not decode PeerList")) // TODO: handle erros properly... again! (Handle error two error cases here)
                }
            }
            Ok(ProtocolMessage::GetBlocks) => {
                let blocks = Encoder::encode(ProtocolMessage::SendBlockchain, self.id, &self.blockchain)?;
                // stream.write(&blocks)?;
                Ok(blocks)
            },
            Ok(ProtocolMessage::SendBlockchain) => {
                println!("Received SendBloclchain message..");
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::SendBlockchain);
                let decoded = decoder.decode_json();
                match decoded {
                    Ok(DecodedType::Blockchain(blockchain)) => {
                        assert!(Blockchain::is_chain_valid(&blockchain));
                        println!("Received new chain... Updating own chain with: {:?}", blockchain);
                        self.blockchain = blockchain;
                        Ok(vec!())
                    },
                    Err(e) => Err(e),
                    _ => Err(Error::new(ErrorKind::InvalidData, "Wrong decoding type used in SendBlockchain command"))
                }
            },
            Ok(ProtocolMessage::AddTransaction) => {
                let mut decoder = Decoder::new(&mut message[..], ProtocolMessage::AddTransaction);

                let decoded_type = decoder.decode_json()?;
                match decoded_type {
                    DecodedType::BlockData(data) => {
                        let new_block = self.blockchain.generate_next_block(&data)?;

                        let message = Encoder::encode(ProtocolMessage::NewBlock, self.id, &new_block)?;

                        println!("New block: {:?}", new_block);
                        Ok(message)
                    },
                    _ => Err(Error::new(ErrorKind::InvalidData, "Wrong decoding type used in AddTransaction command"))
                }
            },
            Ok(ProtocolMessage::NewBlock) => {
                println!("New Block received");
                Ok(vec!()) //TODO: Should be an option of a vec!
            }
            Err(e) => Err(e),
            Ok(_) => { Err(Error::new(ErrorKind::InvalidData, "No path for given protocol"))}
        }
    }
}