extern crate ws;
use crate::encoder::{Encoder};
use std::sync::{Arc, Mutex};
use crate::node;
use crate::protocol_message::ProtocolMessage;

use ws::{Handler, Sender, Handshake, Result, Message, Error};

// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
pub struct Client {
    out: Sender,
    node: Arc<Mutex<node::Node>>,
}

impl Client {
    pub fn new(out: Sender, node: Arc<Mutex<node::Node>>) -> Client {
        Client {
            out,
            node,
        }
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
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        
        // if it's not the root node then...
        let mut node = self.node.lock().unwrap();
        println!("Sending message from client... {:?}", Encoder::encode(ProtocolMessage::AddMe, node.id, &String::new())?);
        // self.out.broadcast(Encoder::encode(ProtocolMessage::AddMe, node.id, &String::new())?);

        let send_transactions_message = node.send_transactions().unwrap();
        self.out.send(send_transactions_message)?;
        let get_chain_message = node.get_chain().unwrap();
        self.out.send(get_chain_message)?;
        Ok(())
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        println!("Got message: {}", msg);
        let mut node = self.node.lock().unwrap();
        let result = node.handle(&mut msg.into_data()); // this handles incoming... might broadcast...
        match result {
            Ok(data) => {
                if data.len() > 0 {
                    self.out.broadcast(data);
                }
                Ok(())
            },
            Err(_) => Err(Error::new(ws::ErrorKind::Internal, "whoooops"))
        }
        // Err("No result to send..")
        // self.out.
        // self.out.close(CloseCode::Normal)
    }
}