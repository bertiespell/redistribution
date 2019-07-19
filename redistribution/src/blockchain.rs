use std::time::SystemTime;
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::Block;
use crate::hasher;
use crate::encoder;
use encoder::{Encodable, Decodable};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blockchain {
    blocks: VecDeque<Block>
}

impl Blockchain {
    pub fn new() -> Blockchain {
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

    pub fn generate_next_block(block_data: &str, previous_block: &Block) -> Block {
        let timestamp = format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()); // TODO: handle unwrap properly here
        let new_block_index = previous_block.index + 1;
        let hash = hasher::calculate_hash(&new_block_index, &previous_block.hash, &timestamp, block_data);
        Block {
            index: new_block_index,
            timestamp: timestamp,
            data: block_data.to_string(),
            hash: hash,
            previous_hash: previous_block.hash.clone()
        }
    }
}

impl Encodable for Blockchain {
    fn encode(&self) -> Vec<u8> {
        let serialized = serde_json::to_string(&self).unwrap();
        serialized.into_bytes()
    }
}

impl Decodable for Blockchain {
    fn decode(&self, bytes: &Vec<u8>) -> Self {
        let json_string = String::from_utf8(bytes.clone()).unwrap();
        let deserialized: Blockchain = serde_json::from_str(&json_string).unwrap();
        deserialized
    }
}

fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool {
    if previous_block.index + 1 != new_block.index {
        return false;
    } else if previous_block.hash != new_block.previous_hash {
        return false;
    } else if Block::calculate_hash_for_block(new_block) != new_block.hash {
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
        let next_block = Blockchain::generate_next_block("Test block data!", &genesis_block);
        let block_is_valid = is_valid_new_block(&next_block, &genesis_block);
        assert_eq!(block_is_valid, true);
    }

    #[test]
    fn test_chain_validity() {
        let mut blockchain = Blockchain::new();
        let genesis_block = blockchain.genesis_block().unwrap();
        let new_block1 = Blockchain::generate_next_block("Block 1", &genesis_block);
        let new_block2 = Blockchain::generate_next_block("Block 2", &new_block1);
        let new_block3 = Blockchain::generate_next_block("Block 3", &new_block2);

        blockchain.add_block(new_block1);
        blockchain.add_block(new_block2);
        blockchain.add_block(new_block3);

        let validity = is_chain_valid(&blockchain);
        assert_eq!(validity, true);
    }
}
