use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use blockchain::{Blockchain, Encodable};
use std::thread;
use std::sync::{Mutex, Arc};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {

    let ROOT_NODE: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7878);

    // initialise
    let blockchain = Blockchain::new();

    // process args
    let port: u16 = match std::env::args().nth(1).map(|a| a.parse()) {
        Some(Ok(n)) => n,
        _ => panic!("Enter a valid port"),
    };
    let address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let socket = Arc::new(Mutex::new(address));

    let socket_1 = Arc::clone(&socket);
    println!("Server running at: {}", &address);
    let listening_thread = thread::spawn(move || {
        let listener = TcpListener::bind(*Arc::clone(&socket_1).lock().unwrap()).unwrap();
        for stream in listener.incoming() {
            handle_connection(stream.unwrap(), &blockchain);
        }
    });

    if address != ROOT_NODE { // TODO: think about whether this should be in a thread - and have a handler as a wrapper
        let mut stream = TcpStream::connect(ROOT_NODE).unwrap();
        println!("{:?}", stream);
        let get_blocks = b"GET /blocks TCP/1.1\r\n"; // send getblockchain
        stream.write(get_blocks);
        stream.read(&mut [0; 128]);
    }

    listening_thread.join().unwrap();
}

fn handle_connection(mut stream: TcpStream, blockchain: &Blockchain) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get_blocks = b"GET /blocks TCP/1.1\r\n"; // send getblockchain
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

/*
let blocks = blockchain.encode();
let json = blockchain.decode(&blocks);
println!("Received get request. Sending: {:?}", &blocks);
let response = format!("{},{:?}", "HTTP/1.1 200 OK\r\n\r\n", &json);
stream.write(&response.as_bytes()).unwrap();
stream.flush().unwrap();
 */