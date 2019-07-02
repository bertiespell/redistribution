use openssl::sha;
use hex;
fn main() {
    println!("Hello, world!");
}

struct Block {
    index: u32, // height of the blockchain
    timestamp: String,
    data: u32,
    hash: String,
    previous_hash: String
}

fn calculate_hash(index: &i32, previous_hash: &str, timestamp: &str, data: &str) -> String {
    let mut hasher = sha::Sha256::new();
    hasher.update(&index.to_be_bytes());
    hasher.update(&previous_hash.as_bytes());
    hasher.update(&timestamp.as_bytes());
    hasher.update(&data.as_bytes());

    let hash = hasher.finish();
    hex::encode(hash)
}