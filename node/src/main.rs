use std::env;
use std::thread;
use std::process;
use std::sync::{Arc};
use std::rc::Rc;
use std::cell::Cell;

extern crate ws;
use ws::{connect, listen};

mod node;
mod config;
mod protocol_message;
mod encoder;
mod decoder;
mod peerlist;
mod server;
mod client;

static ROOT_NODE: &str = "127.0.0.1:7878";

fn main() {
  // Cell gives us interior mutability so we can increment
  // or decrement the count between handlers.
  // Rc is a reference-counted box for sharing the count between handlers
  // since each handler needs to own its contents.
  let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let node = node::Node::new().unwrap();
  
    let cloned_node = Arc::clone(&node);
    let listening_thread = thread::spawn(move || {
        let count = Rc::new(Cell::new(0));
        listen(config.address, |out| { 
            let cloned_again = Arc::clone(&cloned_node);
            server::Server::new(out, count.clone(), cloned_again)
        }).unwrap();
    });

    if config.address != ROOT_NODE.parse().unwrap() { 
        let mut url = String::from("ws://");
        url.push_str(&ROOT_NODE.parse::<String>().unwrap());
        connect(url, |out| {
            let another_clone = Arc::clone(&node);
            client::Client::new(out, another_clone)
        }).unwrap();
    }

    listening_thread.join().unwrap();
} 