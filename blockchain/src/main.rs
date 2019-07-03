use openssl::sha;
use hex;
use std::time::SystemTime;

fn main() {
}

#[derive(Clone, Debug)]
struct Block {
    index: u32, // height of the blockchain
    timestamp: String,
    data: String,
    hash: String,
    previous_hash: String
}

impl Block {
    fn genesis_block() -> Block {
    let timestamp = format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()); // TODO: handle unwrap properly here
        let hash = calculate_hash(&0, &String::new(), &timestamp, &String::new());
        Block {
            index: 0,
            timestamp: timestamp,
            data: String::new(),
            hash: hash,
            previous_hash: String::new()
        }
    }
}

fn generate_next_block(block_data: &str, previous_block: &Block) -> Block {
    let timestamp = format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()); // TODO: handle unwrap properly here
    let new_block_index = previous_block.index + 1;
    let hash = calculate_hash(&new_block_index, &previous_block.hash, &timestamp, block_data);
    Block {
        index: new_block_index,
        timestamp: timestamp,
        data: block_data.to_string(),
        hash: hash,
        previous_hash: previous_block.hash.clone()
    }
}

struct Blockchain {
    blocks: Vec<Block>
}

fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool {
    let calculated_has = calculate_hash_for_block(new_block);
    if previous_block.index + 1 != new_block.index {
        return false;
    } else if previous_block.hash != new_block.previous_hash {
        return false;
    } else if calculated_has != new_block.hash {
        return false;
    }
    true
}

fn calculate_hash(index: &u32, previous_hash: &str, timestamp: &str, data: &str) -> String {
    let mut hasher = sha::Sha256::new();
    hasher.update(&index.to_be_bytes());
    hasher.update(&previous_hash.as_bytes());
    hasher.update(&timestamp.as_bytes());
    hasher.update(&data.as_bytes());

    let hash = hasher.finish();
    hex::encode(hash)
}

fn calculate_hash_for_block(block: &Block) -> String {
    calculate_hash(&block.index, &block.previous_hash, &block.timestamp, &block.data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block_validity() {
        let genesis_block = Block::genesis_block();
        let next_block = generate_next_block(&String::from("Test block data!"), &genesis_block);
        let block_is_valid = is_valid_new_block(&next_block, &genesis_block);
        assert_eq!(block_is_valid, true);
    }
}
