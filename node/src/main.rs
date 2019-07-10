use std::env;
use std::process;
mod client;
mod config;
fn main() {
    let config = config::Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    let client = client::Client::new(config.address);
    client.thread.join().unwrap();
}