use std::io::prelude::*;
use std::net::{TcpStream};
use blockchain::{Blockchain, Encodable};
use std::sync::{Arc};
use std::net::{SocketAddr};

#[derive(Debug)]
pub struct Client {
    blockchain: Blockchain,
    peers: Vec<String>,
}

impl Client {
    pub fn new() -> Arc<Client> {
        let blockchain = Blockchain::new();
        let peers = vec!();

        Arc::new(Client {
            blockchain,
            peers,
        })
    }

    pub fn discover_peers(&self, root: SocketAddr) {
        let mut stream = TcpStream::connect(root).unwrap();
        let get_blocks = b"getBlocks"; // send getblockchain
        stream.write(get_blocks);
        stream.read(&mut [0; 128]);
        // TODO: Write discovery peers to own blockchain
    }

    pub fn handle_incoming(&self, mut stream: TcpStream) {
        // TODO: this should parse different messages and route them appropriately
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        println!("Incoming");
        let get_blocks = b"getBlocks"; // send getblockchain
        let mint_block = b"mintBlock"; // generated next block and send it
        let get_peers = b"peers"; // iterate over sockets
        let add_peer = b"addPeer"; // connect to peers

        if buffer.starts_with(get_blocks) {
            let blocks = self.blockchain.encode(); // TODO: Let's handle this - blockchain might not be initialised here
            println!("Received get request. Sending: {:?}", &blocks);
            stream.write(&blocks).unwrap();
            stream.flush().unwrap();
        } else if buffer.starts_with(mint_block) {
            //
        } else if buffer.starts_with(get_peers) {
            //
        } else if buffer.starts_with(add_peer) {
            //
        }
    }
}
