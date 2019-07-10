use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::env;

pub struct Config {
    pub address: SocketAddr,
    pub port: u16,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let port = match args.next() {
            Some(port) => {
                match port.parse() {
                    Ok(port) => port,
                    Err(_e) => return Err("Could not parse port number: {}")
                }
            },
            None => return Err("Didn't get a port number")
        };

        let address = match args.next() {
            Some(_address) => {
                println!("Defaulting to local host..."); // TODO: this should actually use a different port
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
            },
            None => {
                println!("Defaulting to local host...");
                SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
            }
        };

        Ok(Config { address, port })
    }
}

pub fn parse_config(args: env::Args) -> Result<Config, &'static str> {
    Config::new(args)
}