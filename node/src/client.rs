use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use blockchain::{Blockchain, Encodable};
use std::thread;
use std::sync::{Arc};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[derive(Debug)]
pub struct Client {
    blockchain: Blockchain,
    peers: Vec<String>,
    root: SocketAddr
}

impl Client {
    pub fn new(address: SocketAddr) -> Arc<Client> {
        let ROOT_NODE: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878); // TODO: The root node should live somewhere more sensible. Maybe a file.

        let blockchain = Blockchain::new();
        let peers = vec!();

        let client = Arc::new(Client {
            blockchain,
            peers,
            root: ROOT_NODE
        });

        let copied_client = Arc::clone(&client);
        let thread = thread::spawn(move || {
            let listener = TcpListener::bind(address).unwrap();
            for stream in listener.incoming() {
                Client::handle_incoming(&copied_client, stream.unwrap());
            }
        });

        if address != ROOT_NODE { // TODO: think about whether this should be in a thread - and have a handler as a wrapper
            Client::discover_peers(ROOT_NODE);
        } else {
            println!("Root node initialised.");
        }
        println!("Client initialised on: {}", &address);
        
        thread.join().unwrap();

        client
    }

    fn discover_peers(root: SocketAddr) {
        let mut stream = TcpStream::connect(root).unwrap();
        let get_blocks = b"getBlocks"; // send getblockchain
        stream.write(get_blocks);
        stream.read(&mut [0; 128]);
        // TODO: Write discovery peers to own blockchain
    }

    fn handle_incoming(&self, mut stream: TcpStream) {
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
