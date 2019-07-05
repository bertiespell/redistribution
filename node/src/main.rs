use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.01:7878").unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap());
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get_blocks = b"GET /blocks HTTP/1.1\r\n"; // send getblockchain
    let mint_block = b"POST /mintBlock HTTP/1.1\r\n"; // generated next block and send it
    let get_peers = b"GET /peers HTTP/1.1\r\n"; // iterate over sockets
    let add_peer = b"POST /addPeer HTTP/1.1\r\n"; // connect to peers
}