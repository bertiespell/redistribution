use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

extern crate ws;
use ws::{CloseCode, Error, Handler, Handshake, Message, Result, Sender};

use crate::node;

pub struct Server {
    out: Sender,
    count: Rc<Cell<u32>>,
    node: Arc<Mutex<node::Node>>,
}

impl Server {
    pub fn new(out: Sender, count: Rc<Cell<u32>>, node: Arc<Mutex<node::Node>>) -> Server {
        Server { out, count, node }
    }
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        println!("Server opened...");
        // shake.remote_addr();
        println!("RESPONSE {:?}", shake.response);
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Tell the user the current count
        println!("Received message {:?}", msg);
        // self.
        println!("The number of live connections is {}", self.count.get());
        // want to get the remote iD here...
        let mut node = self.node.lock().unwrap();
        let result = node.handle(&mut msg.into_data()).unwrap(); // this handles incoming... might broadcast...
        self.out.broadcast(result)
        // Echo the message back
        // self.out.send(result)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away => println!("The client is leaving the site."),
            CloseCode::Abnormal => {
                println!("Closing handshake failed! Unable to obtain closing status from client.")
            }
            _ => println!("The client encountered an error: {}", reason),
        }

        // The connection is going down, so we need to decrement the count
        self.count.set(self.count.get() - 1)
    }

    fn on_error(&mut self, err: Error) {
        println!("The server encountered an error: {:?}", err);
    }
}
