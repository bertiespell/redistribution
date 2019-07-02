use openssl::sha;
use hex;
use std::time::SystemTime;

fn main() {
    println!("Hello, world!");
}

struct Block {
    index: u32, // height of the blockchain
    timestamp: SystemTime,
    data: String,
    hash: String,
    previous_hash: String
}

impl Block {
    fn genesis_block() -> Block {
        let timestamp = SystemTime::now();
        let hash = calculate_hash(&0, &String::new(), &format!("{:?}", timestamp), &String::new());
        Block {
            index: 0,
            timestamp: timestamp,
            data: String::new(),
            hash: hash,
            previous_hash: String::new()
        }
    }
}

fn generate_next_block(block_data: &str, last_block: Block) -> Block {
    let timestamp = SystemTime::now();
    let hash = calculate_hash(&0, &last_block.hash, &format!("{:?}", timestamp), &String::new());
    Block {
        index: 0,
        timestamp: timestamp,
        data: String::new(),
        hash: hash,
        previous_hash: String::new()
    }
}

struct Blockchain {
    blocks: Vec<Block>
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