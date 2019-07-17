use std::env;
use std::thread;
use std::thread::{JoinHandle};
use std::process;
use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

mod client;
mod config;
mod protocol_message;
mod parser;

static ROOT_NODE: &str = "127.0.0.1:7878";

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    let client = client::Client::new();

    let listener_thread = intialise_listener(Arc::clone(&client), config);
    let discovery_thread = initalise_discovery(Arc::clone(&client), config);
    discovery_thread.join().unwrap();
    listener_thread.join().unwrap();
}

/// Starts a listener thread and waits
fn intialise_listener(client: Arc<Mutex<client::Client>>, config: config::Config) -> JoinHandle<()> {
    thread::spawn(move || {
        let listener = TcpListener::bind(config.address).unwrap();
        for stream in listener.incoming() {
            let mut client = client.lock().unwrap();
            client.handle_incoming(stream.unwrap());
        }
    })
}

/// startes a thread to discover peers
fn initalise_discovery(client: Arc<Mutex<client::Client>>, config: config::Config) -> JoinHandle<()> {
    thread::spawn(move || {
        if config.address != ROOT_NODE.parse().unwrap() {
            let mut client = client.lock().unwrap();
            let add_me_stream = TcpStream::connect(ROOT_NODE).unwrap();
            let initialised_client = client.add_me(add_me_stream);
            match initialised_client {
                Ok(_) => {
                    let get_peers_stream = TcpStream::connect(ROOT_NODE).unwrap();
                    let peers_found = client.get_peers(get_peers_stream);
                    match peers_found {
                        Ok(_) => {},
                        Err(e) => eprintln!("Failed to get list of peers: {}", e)
                    }
                },
                Err(e) => eprintln!("Failed to connect to root: {}", e)
            }
            
        } else {
            // TODO: the root node needs an ID!
            println!("Root node initialised.");
        }
        println!("Client initialised on: {}", &config.address);
    })
}