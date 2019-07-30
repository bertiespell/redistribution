extern crate ws;
use crate::node;
use std::sync::{Arc, Mutex};

use ws::{Error, ErrorKind, Handler, Handshake, Message, Result, Sender};

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
pub struct Client {
    out: Sender,
    node: Arc<Mutex<node::Node>>,
}

impl Client {
    pub fn new(out: Sender, node: Arc<Mutex<node::Node>>) -> Client {
        Client { out, node }
    }
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {
    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        println!("CLIENT: Opening new connection to: {:?}", shake.peer_addr);

        let mut node = self.node.lock().unwrap();

        // initial set-up involves a request to be added to the networ
        let add_me_message = node.add_me().unwrap();
        self.out.send(add_me_message)?;
        // then discovering and adding peers
        let get_peers_message = node.get_peers().unwrap();
        self.out.send(get_peers_message)?;

        // TODO: handle the pruning and selection of peers

        let send_transactions_message = node.send_transactions().unwrap();
        self.out.send(send_transactions_message)?;
        let get_chain_message = node.get_chain().unwrap();
        self.out.send(get_chain_message)?;
        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        let mut node = self.node.lock().unwrap();
        let result = node.handle_message(&mut msg.into_data());
        match result {
            Ok(message) => {
                match message.connect {
                    Some(connection) => {
                        let mut peer_url = String::from("ws://");
                        peer_url.push_str(&connection.to_string());
                        let url = url::Url::parse(&peer_url).unwrap();
                        self.out.connect(url)?;
                    }
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
}
