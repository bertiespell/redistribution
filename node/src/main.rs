use std::env;
use std::thread;
use std::process;
use std::sync::{Arc};
mod client;
mod config;

fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    client::Client::new(config.address);
}