use std::io::prelude::*;
use std::io::Result;
use std::net::{TcpStream};
use blockchain::{Blockchain, Encodable};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use serde_json;

use crate::protocol_message::ProtocolMessage;

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
        let message = build_message(ProtocolMessage::GetPeers, &self.id.unwrap());

        stream.write(&message[..])?;

        let mut buffer = vec!();
        let result = stream.read_to_end(&mut buffer);
        match result {
            Ok(_) => {
                // decode buffer - serialisatble structure
                let decoded_JSON = String::from_utf8(buffer).unwrap();
                let peers: Vec<u128> = serde_json::from_str(&decoded_JSON).unwrap();
                self.peers = peers;
                println!("Received Peers: {:?}", &self.peers);
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
            let peers = serde_json::to_string(&self.peers).unwrap();
            stream.write(&peers.as_bytes()).unwrap();
            stream.flush().unwrap();
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

fn build_message<T: ?Sized>(opcode: ProtocolMessage, message: &T) -> Vec<u8>
where
    T: Serialize
{
    let mut op_code = vec![opcode.as_str()];
    let serialised_value = serde_json::to_string(message).unwrap();
    op_code.push(&serialised_value);
    
    op_code
        .into_iter()
        .map(|astring| astring.as_bytes().to_owned())
        .flatten()
        .collect::<Vec<_>>()
}