use std::cell::Cell;
use std::env;
use std::process;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

extern crate ws;
use ws::{connect, listen};

mod client;
mod config;
mod decoder;
mod encoder;
mod node;
mod peerlist;
mod protocol_message;
mod server;

static ROOT_NODE: &str = "127.0.0.1:7878";

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    let node = node::Node::new(config.address.to_string());

    let cloned_node = Arc::clone(&node);
    let listening_thread = thread::spawn(move || {
        let count = Rc::new(Cell::new(0));
        listen(config.address, |out| {
            let cloned_again = Arc::clone(&cloned_node);
            server::Server::new(out, count.clone(), cloned_again)
        })
        .unwrap();
    });

    if config.address != ROOT_NODE.parse().unwrap() {
        let mut url = String::from("ws://");
        url.push_str(&ROOT_NODE.parse::<String>().unwrap());
        connect(url, |out| {
            let another_clone = Arc::clone(&node);
            client::Client::new(out, another_clone)
        })
        .unwrap();
    }

    listening_thread.join().unwrap();
}
