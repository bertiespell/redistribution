use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use blockchain::{Blockchain, Encodable};
use std::thread;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub struct Client {
    pub thread: thread::JoinHandle<()>,
    blockchain: Option<Blockchain>,
    peers: Vec<String>,
    root: SocketAddr
}

impl Client {
    pub fn new(address: SocketAddr) -> Self {
        let ROOT_NODE: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);

        let blockchain = Blockchain::new();

        let thread = thread::spawn(move || {
            let listener = TcpListener::bind(address).unwrap();
            for stream in listener.incoming() {
                Client::handle_incoming(stream.unwrap());
            }
        });

        let mut peers = vec!();

        if address != ROOT_NODE { // TODO: think about whether this should be in a thread - and have a handler as a wrapper
            Client::discover_peers(ROOT_NODE);
        }
        println!("Client initialised on: {}", &address);
        Client {
            thread,
            blockchain: Some(blockchain),
            peers,
            root: ROOT_NODE
        }
    }

    fn discover_peers(root: SocketAddr) {
        let mut stream = TcpStream::connect(root).unwrap();
        println!("{:?}", stream);
        let get_blocks = b"GET /blocks TCP/1.1\r\n"; // send getblockchain
        stream.write(get_blocks);
        stream.read(&mut [0; 128]);
        // TODO: Write discovery peers to own blockchain
    }

    fn handle_incoming(&self, mut stream: TcpStream) {
        // TODO: this should parse different messages and route them appropriately
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let get_blocks = b"GET /blocks TCP/1.1\r\n"; // send getblockchain
        let mint_block = b"POST /mintBlock HTTP/1.1\r\n"; // generated next block and send it
        let get_peers = b"GET /peers HTTP/1.1\r\n"; // iterate over sockets
        let add_peer = b"POST /addPeer HTTP/1.1\r\n"; // connect to peers

        if buffer.starts_with(get_blocks) {
            let blocks = self.blockchain.unwrap().encode(); // TODO: Let's handle this - blockchain might not be initialised here
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
