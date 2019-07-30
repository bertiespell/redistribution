use std::cell::Cell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

extern crate ws;
use ws::{CloseCode, Error, ErrorKind, Handler, Handshake, Message, Result, Sender};
use url::Url;
// use url::{Url, Host};
// extern crate url;



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
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // We have a new connection, so we increment the connection counter
        Ok(self.count.set(self.count.get() + 1))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("The number of live connections is {}", self.count.get());
        let mut node = self.node.lock().unwrap();
        let result = node.handle_message(&mut msg.into_data());
        match result {
            Ok(message) => {
                match message.connect {
                    Some(connection) => {
                        // connect to it....
                        let mut peer_url = String::from("ws://");
                        peer_url.push_str(&connection.to_string());
                        let url = url::Url::parse(&peer_url).unwrap();
                        self.out.connect(url)?;
                    },
                    _ => {}
                }
                if message.broadcast {
                    match message.raw_message {
                        Some(data) => {
                            return self.out.broadcast(data);
                        }
                        None => {
                            return Err(Error::new(
                                ErrorKind::Internal,
                                "Asking to broadcast message with no data",
                            ));
                        }
                    }
                } else {
                    match message.raw_message {
                        Some(data) => {
                            return self.out.send(data);
                        }
                        _ => {}
                    }
                }
                
                Ok(())
            }
            Err(e) => Err(Error::from(e)),
        }
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
