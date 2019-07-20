use std::io::prelude::*;
use std::io::{Result, Error, ErrorKind};
use std::net::{TcpStream, Shutdown};
use redistribution::{Blockchain, Encodable};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::net::{SocketAddr};

use crate::parser::{Decoder, Encoder, DecodedType};
use crate::protocol_message::ProtocolMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
	pub id: u128,
    blockchain: Blockchain,
    peers: HashMap<u128, SocketAddr>, // list of IDs
}

impl Node {
    pub fn new() -> Arc<Mutex<Node>> {
        let blockchain = Blockchain::new();
        let peers = HashMap::new();

        Arc::new(Mutex::new(Node {
			id: 0,
            blockchain,
            peers,
        }))
    }

    pub fn add_me(&mut self, mut stream: TcpStream) -> Result<()> {
        // TODO: Make into properly encoded thingy...

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
                    Ok(DecodedType::Node_ID(node_id)) => {
                        self.id = node_id;
                        println!("Received ID: {:?}", &node_id);
                        Ok(())
                    },
                    Err(e) => {
                        println!("Decoder Error");
                        Err(Error::new(ErrorKind::Other, "Decoder Error"))
                    },
                    _ => {
                        println!("Wrong decoding typ received");
                        Err(Error::new(ErrorKind::Other, "Wrong decoding typ received"))
                    }
                }
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_peers(&mut self, mut stream: TcpStream) -> Result<()> {
        // TODO: Put this in an encoder...
        let message = Encoder::encode(ProtocolMessage::GetPeers, self.id, &String::new());

        println!("Sending: {:?}", &message[..]);
        stream.write(&message[..])?;

        let mut buffer = vec!();
        let result = stream.read_to_end(&mut buffer);
        match result {
            Ok(_) => {
                // decode buffer - serialisatble structure
                let decoded_json = String::from_utf8(buffer).unwrap();
                let peers: HashMap<u128, SocketAddr> = serde_json::from_str(&decoded_json).unwrap();
                self.peers = peers;
                println!("Received Peers: {:?}", &self.peers);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn send_transactions(&self, mut stream: TcpStream) {
        // TODO: put this in an encoder.
        let transaction = String::from("hello"); // TODO: this should be actual data!
        let message = Encoder::encode(ProtocolMessage::AddTransaction, self.id, &transaction);
        
        println!("Sending transations: {:?}", &message[..]);
        stream.write(&message[..]);
        let mut buffer = [0; 16];
        let result = stream.read(&mut buffer);
        // TODO: Properly decode mined block - then actually do something with it!

        println!("Received new block: {:?}", buffer);
    }

    pub fn handle_incoming(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let opcode = Decoder::opcode(&mut buffer);
        
        match opcode {
            Ok(ProtocolMessage::AddMe) => {
                // TODO: ensure we're using UUID. Here we just use an incrementing ID - ideally in the future one node won't store *all* other nodes in its peers... so we'll need a smarter system
                let node_addr = stream.peer_addr().unwrap();
                let mut highest_id: u128 = 1;
                let mut peers = self.peers.iter();

                while let Some((peer_id, _)) = peers.next() {
                    if highest_id < *peer_id {
                        highest_id = *peer_id;
                    }
                }

                highest_id = highest_id + 1;
                self.peers.insert(highest_id, node_addr);

                let message = Encoder::encode(ProtocolMessage::AddedPeer, self.id, &highest_id);
                println!("Sending ID message {:?}", &message);
                stream.write(&message).unwrap();
                // TODO: Broadcast new node to network?
            },
            Ok(ProtocolMessage::GetPeers) => {
                let mut decoder = Decoder::new(buffer, ProtocolMessage::GetPeers);
                let peer = decoder.peer_id();
                println!("Peer ID: {}", peer);
                assert!(self.peers.contains_key(&peer));
                if self.peers.contains_key(&peer) {
                    println!("Received GetPeer request from: {}", peer);
                    let peers = serde_json::to_string(&self.peers).unwrap();
                    // TODO: Now this should be encoded
                    stream.write(&peers.as_bytes()).unwrap();
                    stream.flush().unwrap();
                } else {
                    println!("Peer not recognised");
                    stream.shutdown(Shutdown::Both).unwrap_or_else(|_| println!("Failed to close connection for unrecognised peer"));
                }
            },
            Ok(ProtocolMessage::GetBlocks) => {
                let blocks = self.blockchain.encode();
                println!("Sending blocks {:?}", &blocks);
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