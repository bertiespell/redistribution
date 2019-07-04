use openssl::sha;
use hex;
use std::time::SystemTime;
use std::collections::VecDeque;

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

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.timestamp == other.timestamp && self.data == other.data && self.hash == other.hash && self.previous_hash == other.previous_hash
    }
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

pub struct Blockchain {
    blocks: VecDeque<Block>
}

impl Blockchain {
    fn new() -> Blockchain {
        let genesis_block = Block::genesis_block();
        let mut blocks = VecDeque::new();
        blocks.push_back(genesis_block);

        Blockchain { 
            blocks
        }
    }

    fn genesis_block(&self) -> Option<&Block> {
        self.blocks.front()
    }

    fn add_block(&mut self, block: Block) {
        // check that the block is valid here
        assert!(is_valid_new_block(&block, self.blocks.back().unwrap()));
        self.blocks.push_back(block);
    }
}

fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool {
    if previous_block.index + 1 != new_block.index {
        return false;
    } else if previous_block.hash != new_block.previous_hash {
        return false;
    } else if calculate_hash_for_block(new_block) != new_block.hash {
        return false;
    }
    true
}

fn is_chain_valid(blockchain: &Blockchain) -> bool {
    // TODO: need to check genesis block somehow   
    blockchain.blocks
        .iter()
        .skip(1)
        .zip(blockchain.blocks.iter())
        .map(|(block, last_block)| block.previous_hash == last_block.hash)
        .fold(true, |x, y| x && y)
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

fn determine_longest_chain<'a>(first_blockchain: &'a Blockchain, second_blockchain: &'a Blockchain) -> &'a Blockchain {
    if first_blockchain.blocks.back().unwrap().index > second_blockchain.blocks.back().unwrap().index {
        return first_blockchain;
    }
    second_blockchain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block_validity() {
        let genesis_block = Block::genesis_block();
        let next_block = generate_next_block("Test block data!", &genesis_block);
        let block_is_valid = is_valid_new_block(&next_block, &genesis_block);
        assert_eq!(block_is_valid, true);
    }

    #[test]
    fn test_chain_validity() {
        let mut blockchain = Blockchain::new();
        let genesis_block = blockchain.genesis_block().unwrap();
        let new_block1 = generate_next_block("Block 1", &genesis_block);
        let new_block2 = generate_next_block("Block 2", &new_block1);
        let new_block3 = generate_next_block("Block 3", &new_block2);

        blockchain.add_block(new_block1);
        blockchain.add_block(new_block2);
        blockchain.add_block(new_block3);

        let validity = is_chain_valid(&blockchain);
        assert_eq!(validity, true);
    }
}
