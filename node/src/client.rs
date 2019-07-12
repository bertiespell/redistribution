use std::io::prelude::*;
use std::net::{TcpStream};
use blockchain::{Blockchain, Encodable};
use std::sync::{Arc};
use std::net::{SocketAddr};
extern crate tokio;
use crate::protocol_message::ProtocolMessage;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[derive(Debug)]
pub struct Client {
	id: Option<u128>,
    blockchain: Blockchain,
    peers: Vec<u128>, // list of IDs
}

impl Client {
    pub fn new() -> Arc<Client> {
        let blockchain = Blockchain::new();
        let peers = vec!();

        Arc::new(Client {
			id: None,
            blockchain,
            peers,
        })
    }

	pub fn initialise(&self, root: SocketAddr) {
		let mut stream = TcpStream::connect(root).unwrap();
		let add_me = ProtocolMessage::AddMe.as_str();
        stream.write(add_me.as_bytes());
        stream.read(&mut [0; 128]);
        
        //TODO:
		// send add me message - recieves ID
		// ORIGINAL/ROOT node - adds ID to list and broadcasts result to everyone
		// New peer with new ID asks for list of nodes...
		// sends back peers
	}

    // TODO: All these tcp streams are repeated - they should be in a wrapper? Using Tokio maybe?
    fn add_me(&self, root: SocketAddr) {
        let mut stream = TcpStream::connect(root).unwrap();
		let add_me = ProtocolMessage::AddMe.as_str();
        stream.write(add_me.as_bytes());
        stream.read(&mut [0; 128]);
    }

    fn get_peers(&self, root: SocketAddr) {
        let mut stream = TcpStream::connect(root).unwrap();
        let get_peers = ProtocolMessage::GetPeers.as_str();
        stream.write(get_peers.as_bytes());
        stream.read(&mut [0; 128]);
        // TODO: Write peers to own hashtable
    }

    pub fn handle_incoming(&self, mut stream: TcpStream) {
        // TODO: this should parse different messages and route them appropriately
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        if buffer.starts_with(ProtocolMessage::GetBlocks.as_str().as_bytes()) {
            let blocks = self.blockchain.encode();
            println!("Received get request. Sending: {:?}", &blocks);
            stream.write(&blocks).unwrap();
            stream.flush().unwrap();
        } else if buffer.starts_with(ProtocolMessage::MintBlock.as_str().as_bytes()) {
            //
        } else if buffer.starts_with(ProtocolMessage::GetPeers.as_str().as_bytes()) {
            // check the node is known in hash table...
            // send back list of peers
        } else if buffer.starts_with(ProtocolMessage::AddMe.as_str().as_bytes()) {
			// TODO: ensure we're using UUID. Here we just use an incrementing ID - ideally in the future one node won't store *all* other nodes in its peers... so we'll need a smarter system
			let mut highest_id: u128 = 0;
			let mut peers = self.peers.iter();

            while let Some(peer_id) = peers.next() {
                if highest_id < *peer_id {
                    highest_id = *peer_id;
                }
            }

			highest_id = highest_id + 1;
            println!("Sending new client id: {}", highest_id);
			stream.write(&highest_id.to_be_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}
