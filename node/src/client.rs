use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpStream};
use blockchain::{Blockchain, Encodable, Decodable};
use std::sync::{Arc, Mutex};
use std::cell::{RefCell, RefMut};
use serde::{Serialize, Deserialize};
use serde_json;

use std::net::{SocketAddr};
use std::rc::Rc;
extern crate tokio;
use crate::protocol_message::ProtocolMessage;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
	id: Option<u128>,
    blockchain: Blockchain,
    peers: Vec<u128>, // list of IDs
}

fn parse_buffer(buffer: &[u8]) -> (&[u8], &[u8]) {
    let opcode = &buffer[0..4];
    let data = &buffer[4..];
    return (opcode, data);
}

impl Client {
    pub fn new() -> Arc<Mutex<Client>> {
        let blockchain = Blockchain::new();
        let peers = vec!();

        Arc::new(Mutex::new(Client {
			id: None,
            blockchain,
            peers,
        }))
    }

	pub fn initialise(&mut self, mut stream: TcpStream) -> Result<()> {
		let rc_stream = Rc::new(RefCell::new(stream));
        self.add_me(rc_stream.borrow_mut())?;
        self.get_peers(rc_stream.borrow_mut())?; // TODO: needs some async stuff here...
        Ok(())
	}

    // TODO: All these tcp streams are repeated - they should be in a wrapper? Using Tokio maybe?
    fn add_me(&mut self, mut stream: RefMut<TcpStream>) -> Result<()> {
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

    fn get_peers(&mut self, mut stream: RefMut<TcpStream>) -> Result<()> {
        let mut newer_thing = vec![ProtocolMessage::GetPeers.as_str()];
        let serialised_value = serde_json::to_string(&self.id.unwrap()).unwrap();
        newer_thing.push(&serialised_value);
        
        let byte_array = newer_thing
            .into_iter()
            .map(|astring| astring.as_bytes().to_owned())
            .flatten()
            .collect::<Vec<_>>();

        stream.write(&byte_array[..])?;

        let mut buffer = vec!();
        let result = stream.read_to_end(&mut buffer);
        println!("Got peers: {:?}", buffer);
        match result {
            Ok(_) => {
                // decode buffer - serialisatble structure
                println!("Received Peers: {:?}", &buffer);
                Ok(())
            },
            Err(e) => Err(e),
        }
    }

    pub fn handle_incoming(&mut self, mut stream: TcpStream) {
        // TODO: this should parse different messages and route them appropriately
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let (opcode, data) = parse_buffer(&buffer);
        println!("OPCODE: {:?}, DATA: {:?}", opcode, data);

        if buffer.starts_with(ProtocolMessage::GetBlocks.as_bytes()) {
            let blocks = self.blockchain.encode();
            println!("Sending blocks {:?}", &blocks);
            stream.write(&blocks).unwrap();
            stream.flush().unwrap();
        } else if buffer.starts_with(ProtocolMessage::MintBlock.as_bytes()) {
            //
        } else if buffer.starts_with(ProtocolMessage::GetPeers.as_bytes()) {
            println!("Received get peers request");
            // check that the node is known
            // Get Node ID from the buffer...
            // check the node is known in hash table...
            // send back list of peers
        } else if buffer.starts_with(ProtocolMessage::AddMe.as_bytes()) {
			// TODO: ensure we're using UUID. Here we just use an incrementing ID - ideally in the future one node won't store *all* other nodes in its peers... so we'll need a smarter system
			let mut highest_id: u128 = 0;
			let mut peers = self.peers.iter();

            while let Some(peer_id) = peers.next() {
                if highest_id < *peer_id {
                    highest_id = *peer_id;
                }
            }

			highest_id = highest_id + 1;
            self.peers.push(highest_id);
            println!("Sending new client id: {:?}", &highest_id);
			stream.write(&highest_id.to_be_bytes()).unwrap();
            // TODO: Broadcast new node to network?
        }
    }
}
