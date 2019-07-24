use std::io::prelude::*;
use std::io::{Result, Error, ErrorKind};
use std::net::{TcpStream, Shutdown};
use redistribution::{Blockchain};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

use crate::encoder::{Encoder};
use crate::decoder::{Decoder, DecodedType};
use crate::peerlist;

use crate::protocol_message::ProtocolMessage;
use peerlist::PeerList;

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
                let mut decoder = Decoder::new(buffer, ProtocolMessage::AddedPeer);
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

    pub fn send_transactions(&self, stream: &mut TcpStream) -> Result<()> {
        let transaction = String::from("hello"); // TODO: this should be actual data!
        let message = Encoder::encode(ProtocolMessage::AddTransaction, self.id, &transaction)?;
        
        stream.write(&message[..])?;
        let mut buffer = [0; 16];
        stream.read(&mut buffer)?;
        // TODO: Properly decode mined block - then actually do something with it!

        println!("Received new block: {:?}", buffer);
        Ok(())
    }

    pub fn get_chain(&mut self, stream: &mut TcpStream) -> Result<()>  {
        let message = Encoder::encode(ProtocolMessage::GetBlocks, self.id, &String::new())?;
        
        stream.write(&message[..])?;

        let mut buffer = [0; 512];
        stream.read(&mut buffer)?;
        let mut decoder = Decoder::new(buffer, ProtocolMessage::SendBlockchain);
        let decoded = decoder.decode_json();
        match decoded {
            Ok(DecodedType::Blockchain(blockchain)) => {
                assert!(Blockchain::is_chain_valid(&blockchain));
                println!("Received new chain... Updating own chain with: {:?}", blockchain);
                self.blockchain = blockchain;
                Ok(())
            },
            Err(e) => Err(e),
            _ => Err(Error::new(ErrorKind::InvalidData, "Wrong decoding type used in SendBlockchain command"))
        }
    }

    pub fn handle_incoming(node: &Arc<Mutex<Node>>, stream: &mut TcpStream) -> Result<()> {
        let mut buffer = [0; 512];
        let num_bytes = stream.read(&mut buffer)?;
        if num_bytes == 0 {
            return Err(Error::new(ErrorKind::ConnectionAborted, "Received 0 bytes from stream - connection aborted"));
        }
        let opcode = Decoder::protocol(&mut buffer); // TODO: Make buffer as long as headers - then allocate dynamic array buffer for the data

        let mut node = node.lock().unwrap();
        match opcode {
            Ok(ProtocolMessage::AddMe) => {
                // TODO: ensure we're using UUID. Here we just use an incrementing ID - ideally in the future one node won't store *all* other nodes in its peers... so we'll need a smarter system
                let node_addr = stream.peer_addr()?;
                let mut highest_id: u128 = 1;
                let mut peers = node.peerlist.peers.iter();

                while let Some((peer_id, _)) = peers.next() {
                    if highest_id < *peer_id {
                        highest_id = *peer_id;
                    }
                }

                highest_id = highest_id + 1;
                node.peerlist.peers.insert(highest_id, node_addr);

                let message = Encoder::encode(ProtocolMessage::AddedPeer, node.id, &highest_id)?;
                stream.write(&message)?;
                // TODO: Broadcast new node to network?
                Ok(())
            },
            Ok(ProtocolMessage::GetPeers) => {
                let mut decoder = Decoder::new(buffer, ProtocolMessage::GetPeers);
                let peer = decoder.peer_id();
                if node.peerlist.peers.contains_key(&peer) {
                    let message = Encoder::encode(ProtocolMessage::PeerList, node.id, &node.peerlist)?;

                    stream.write(&message)?;
                    Ok(())
                } else {
                    stream.shutdown(Shutdown::Both)?;
                    Err(Error::new(ErrorKind::InvalidData, "Message from unrecognised peer"))
                }
            },
            Ok(ProtocolMessage::GetBlocks) => {
                let blocks = Encoder::encode(ProtocolMessage::SendBlockchain, node.id, &node.blockchain)?;
                stream.write(&blocks)?;
                Ok(())
            },
            Ok(ProtocolMessage::AddTransaction) => {
                let mut decoder = Decoder::new(buffer, ProtocolMessage::AddTransaction);

                let decoded_type = decoder.decode_json()?;
                match decoded_type {
                    DecodedType::BlockData(data) => {
                        let new_block = node.blockchain.generate_next_block(&data)?;

                        let message = Encoder::encode(ProtocolMessage::NewBlock, node.id, &new_block)?;

                        println!("New block: {:?}", new_block);

                        stream.write(&message)?;
                        Ok(())
                    },
                    _ => Err(Error::new(ErrorKind::InvalidData, "Wrong decoding type used in AddTransaction command"))
                }
            },
            Err(e) => Err(e),
            Ok(_) => { Err(Error::new(ErrorKind::InvalidData, "No path for given protocol"))}
        }
    }
}