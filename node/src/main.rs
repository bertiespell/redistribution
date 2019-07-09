use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use blockchain::{Blockchain, Encodable, Decodable};
use std::env;
use std::thread;
use std::sync::Arc;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};


static NODE_SEED: &str = "127.0.01:7878";

fn main() {

    // initialise
    let blockchain = Blockchain::new();

    // process args
    // let args: Vec<String> = env::args().collect();
    let port: u16 = match std::env::args().nth(1).map(|a| a.parse()) {
        Some(Ok(n)) => n,
        _           => panic!("Enter a valid port"),
    };
    let socket = Arc::new(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port));

    // println!("Server running at: {}", address);
    thread::spawn(move || {
        let listener = TcpListener::bind(*Arc::clone(&socket)).unwrap();
        for stream in listener.incoming() {
            handle_connection(stream.unwrap(), &blockchain);
        }
    });

    // This function will block the calling thread until a new TCP connection is established. When established, the corresponding TcpStream and the remote peer's address will be returned.
        let listener = TcpListener::bind(*Arc::clone(&socket)).unwrap();

    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }
}

fn handle_connection(mut stream: TcpStream, blockchain: &Blockchain) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get_blocks = b"GET /blocks HTTP/1.1\r\n"; // send getblockchain
    let mint_block = b"POST /mintBlock HTTP/1.1\r\n"; // generated next block and send it
    let get_peers = b"GET /peers HTTP/1.1\r\n"; // iterate over sockets
    let add_peer = b"POST /addPeer HTTP/1.1\r\n"; // connect to peers

    if buffer.starts_with(get_blocks) {
        let blocks = blockchain.encode();
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