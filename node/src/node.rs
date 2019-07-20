use std::io::prelude::*;
use std::io::{Result, Error, ErrorKind};
use std::net::{TcpStream, Shutdown};
use redistribution::{Blockchain, Encodable, Decodable};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::net::{SocketAddr};

use crate::encoder::{Encoder};
use crate::decoder::{Decoder, DecodedType};

use crate::protocol_message::ProtocolMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
	pub id: u128,
    blockchain: Blockchain,
    peerlist: PeerList, // list of IDs
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerList {
    peers: HashMap<u128, SocketAddr>
}

impl Encodable for PeerList {
    fn encode(&self) -> Vec<u8> {
        let peers = serde_json::to_string(&self.peers).unwrap();
        peers.into_bytes()
    }
}

impl Decodable for PeerList {
    fn decode(bytes: &Vec<u8>) -> Self {
        let decoded_json = String::from_utf8(bytes.clone()).unwrap();
        let peers: HashMap<u128, SocketAddr> = serde_json::from_str(&decoded_json).unwrap();
        PeerList {
            peers
        }
    }
}

impl Node {
    pub fn new() -> Arc<Mutex<Node>> {
        let blockchain = Blockchain::new();
        let peers = HashMap::new();

        Arc::new(Mutex::new(Node {
			id: 0,
            blockchain,
            peerlist: PeerList {
                peers
            },
        }))
    }

    pub fn add_me(&mut self, mut stream: TcpStream) -> Result<()> {
        let message = Encoder::encode(ProtocolMessage::AddMe, self.id, &String::new());

        stream.write(&message)?;

        let mut buffer = [0; 512];
        let result = stream.read(&mut buffer);
        match result {
            Ok(_) => {
                // TODO: pull the node ID out of the encoder
                let mut decoder = Decoder::new(buffer, ProtocolMessage::AddedPeer);
                let decoder_type = decoder.decode_json();
                match decoder_type {
                    Ok(DecodedType::NodeID(node_id)) => {
                        self.id = node_id;
                        Ok(())
                    },
                    Err(e) => {
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

    pub fn get_peers(&mut self, mut stream: TcpStream) -> Result<()> {
        let message = Encoder::encode(ProtocolMessage::GetPeers, self.id, &String::new());

        stream.write(&message[..])?;

        let mut buffer = [0; 512];
        let result = stream.read(&mut buffer);
        match result {
            Ok(_) => {
                // let decoded_json = String::from_utf8(buffer).unwrap();
                // let peers: HashMap<u128, SocketAddr> = serde_json::from_str(&decoded_json).unwrap();
            
                let mut decoder = Decoder::new(buffer, ProtocolMessage::PeerList);

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

    pub fn send_transactions(&self, mut stream: TcpStream) {
        let transaction = String::from("hello"); // TODO: this should be actual data!
        let message = Encoder::encode(ProtocolMessage::AddTransaction, self.id, &transaction);
        
        stream.write(&message[..]);
        let mut buffer = [0; 16];
        let result = stream.read(&mut buffer);
        // TODO: Properly decode mined block - then actually do something with it!

        println!("Received new block: {:?}", buffer);
    }

    pub fn get_chain(&mut self, mut stream: TcpStream) {
        let message = Encoder::encode(ProtocolMessage::GetBlocks, self.id, &String::new());
        
        stream.write(&message[..]);

        let mut buffer = [0; 512];
        stream.read(&mut buffer);
        let mut decoder = Decoder::new(buffer, ProtocolMessage::SendBlockchain);
        let decoded = decoder.decode_json();
        match decoded {
            Ok(DecodedType::Blockchain(blockchain)) => {
                println!("Got Chain: {:?}", blockchain);
                // TODO: should verify here
                self.blockchain = blockchain;
            },
            _ => println!("Could not decode blockchain")
        }
    }

    pub fn handle_incoming(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let opcode = Decoder::protocol(&mut buffer);
        
        match opcode {
            Ok(ProtocolMessage::AddMe) => {
                // TODO: ensure we're using UUID. Here we just use an incrementing ID - ideally in the future one node won't store *all* other nodes in its peers... so we'll need a smarter system
                let node_addr = stream.peer_addr().unwrap();
                let mut highest_id: u128 = 1;
                let mut peers = self.peerlist.peers.iter();

                while let Some((peer_id, _)) = peers.next() {
                    if highest_id < *peer_id {
                        highest_id = *peer_id;
                    }
                }

                highest_id = highest_id + 1;
                self.peerlist.peers.insert(highest_id, node_addr);

                let message = Encoder::encode(ProtocolMessage::AddedPeer, self.id, &highest_id);
                println!("Sending ID message {:?}", &message);
                stream.write(&message).unwrap();
                // TODO: Broadcast new node to network?
            },
            Ok(ProtocolMessage::GetPeers) => {
                let mut decoder = Decoder::new(buffer, ProtocolMessage::GetPeers);
                let peer = decoder.peer_id();
                assert!(self.peerlist.peers.contains_key(&peer));
                if self.peerlist.peers.contains_key(&peer) {
                    let message = Encoder::encode(ProtocolMessage::PeerList, self.id, &self.peerlist);

                    stream.write(&message).unwrap();
                    stream.flush().unwrap();
                } else {
                    println!("Peer not recognised");
                    stream.shutdown(Shutdown::Both).unwrap_or_else(|_| println!("Failed to close connection for unrecognised peer"));
                }
            },
            Ok(ProtocolMessage::GetBlocks) => {
                let blocks = Encoder::encode(ProtocolMessage::SendBlockchain, self.id, &self.blockchain);
                stream.write(&blocks).unwrap();
                stream.flush().unwrap();
            },
            Ok(ProtocolMessage::AddTransaction) => {
                let mut decoder = Decoder::new(buffer, ProtocolMessage::AddTransaction);

                let decoded = decoder.decode_json().unwrap();
                match decoded {
                    DecodedType::BlockData(data) => {
                        let new_block = self.blockchain.generate_next_block(&data); //TODO: proper error handling - this hsould return an encoded enum type that we can match on

                        let message = Encoder::encode(ProtocolMessage::NewBlock, self.id, &new_block);

                        println!("New block: {:?}", new_block);

                        stream.write(&message).unwrap(); // TODO: This needs to be properly encoded
                        stream.flush().unwrap();
                    },
                    _ => {}
                }
            },
            Err(_) => { println!("Received unknown opcode")},
            _ => { println!("Unimplemented message path")}
        }
    }
}