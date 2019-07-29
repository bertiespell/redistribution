use std::net::{TcpStream};

pub struct Broadcast<'a> {
    streams: Vec<&'a TcpStream>
}

impl <'a> Broadcast<'a> {
    pub fn new(streams: Vec<&'a TcpStream>) -> Broadcast<'a> {
        Broadcast {
            streams
        }
    }

    fn broadcast_message(&self) -> () {

    }

    pub fn add_stream(&mut self, stream: &'a TcpStream) -> () {
        self.streams.push(stream);
    }
}