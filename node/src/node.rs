use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpStream, Shutdown};
use redistribution::{Blockchain, Encodable};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use std::net::{SocketAddr};

use crate::parser::Parser;
use crate::protocol_message::ProtocolMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
	id: Option<u128>,
    blockchain: Blockchain,
    peers: HashMap<u128, SocketAddr>, // list of IDs
}

impl Node {
    pub fn new() -> Arc<Mutex<Node>> {
        let blockchain = Blockchain::new();
        let peers = HashMap::new();

        Arc::new(Mutex::new(Node {
			id: None,
            blockchain,
            peers,
        }))
    }

    pub fn add_me(&mut self, mut stream: TcpStream) -> Result<()> {
        let add_me = ProtocolMessage::AddMe.as_str();
        stream.write(add_me.as_bytes())?;

        let mut buffer = [0; 16];
        let result = stream.read(&mut buffer);
        match result {
            Ok(_) => {
                let node_id = u128::from_be_bytes(buffer);
                self.id = Some(node_id);
                println!("Received ID: {:?}", &node_id);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn get_peers(&mut self, mut stream: TcpStream) -> Result<()> {

        let mut message = vec!();
        let peer_id = self.id.unwrap().to_be_bytes();
        ProtocolMessage::GetPeers.as_bytes().iter().for_each(|x|{message.push(*x)});
        peer_id.iter().for_each(|x|{message.push(*x)});

        Parser::build_raw_message(ProtocolMessage::GetPeers, &self.id.unwrap().to_be_bytes().to_vec());

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

    pub fn send_transactions() {
        // TODO: 
        let transaction = "hello";
        Parser::build_json_message(ProtocolMessage::MintBlock, transaction);
    }

    pub fn handle_incoming(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let opcode = Parser::opcode(&mut buffer);
        
        match opcode {
            Ok(ProtocolMessage::AddMe) => {
                // TODO: ensure we're using UUID. Here we just use an incrementing ID - ideally in the future one node won't store *all* other nodes in its peers... so we'll need a smarter system
                let node_addr = stream.peer_addr().unwrap();
                let mut highest_id: u128 = 0;
                let mut peers = self.peers.iter();

                while let Some((peer_id, _)) = peers.next() {
                    if highest_id < *peer_id {
                        highest_id = *peer_id;
                    }
                }

                highest_id = highest_id + 1;
                self.peers.insert(highest_id, node_addr);
                println!("Sending new node id: {:?}", &highest_id);
                stream.write(&highest_id.to_be_bytes()).unwrap();
                // TODO: Broadcast new node to network?
            },
            Ok(ProtocolMessage::GetPeers) => {
                let mut parser = Parser::new(buffer, ProtocolMessage::GetPeers);
                let peer = parser.peer_id();
                assert!(self.peers.contains_key(&peer));
                if self.peers.contains_key(&peer) {
                    println!("Received GetPeer request from: {}", peer);
                    let peers = serde_json::to_string(&self.peers).unwrap();
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
            Ok(ProtocolMessage::MintBlock) => {},
            Err(_) => { println!("Received unknown opcode")}
        }
    }
}