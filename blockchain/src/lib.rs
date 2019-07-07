use openssl::sha;
use hex;
use std::time::SystemTime;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Block {
    index: u32, // height of the blockchain
    timestamp: String,
    data: Vec<UnverifiedTransaction>,
    hash: String, // TODO: convert this type to a vec<u8>
    nonce: u64,
    previous_hash: String
}

#[derive(Clone, Debug)]
pub struct Transaction {}

#[derive(Clone, Debug)]
pub struct UnverifiedTransaction {
	/// Plain Transaction.
	unsigned: Transaction,
}

impl Block {
	/// Get the RLP-encoding of the block with the seal.
	pub fn rlp_bytes(&self) -> Vec<u8> {
		let mut block_rlp = RlpStream::new_list(6);
		block_rlp.append(&self.index);
		block_rlp.append_list(&self.timestamp);
		block_rlp.append_list(&self.data);
        block_rlp.append(&self.hash);
        block_rlp.append(&self.nonce);
        block_rlp.append(&self.previous_hash);
		block_rlp.out()
	}
}

impl Decodable for Block {
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError> {
		if rlp.as_raw().len() != rlp.payload_info()?.total() {
			return Err(DecoderError::RlpIsTooBig);
		}
		if rlp.item_count()? != 3 {
			return Err(DecoderError::RlpIncorrectListLen);
		}
		Ok(Block {
            index: rlp.val_at(0)?,
            timestamp: rlp.val_at(1)?,
            data: rlp.val_at(2)?,
            hash: rlp.val_at(3)?, 
            nonce: rlp.val_at(4)?,
            previous_hash: rlp.val_at(5)?,
		})
	}
}

use {DecoderError, Rlp, RlpStream};

/// RLP decodable trait
pub trait Decodable: Sized {
	/// Decode a value from RLP bytes
	fn decode(rlp: &Rlp) -> Result<Self, DecoderError>;
}

/// Structure encodable to RLP
pub trait Encodable {
	/// Append a value to the stream
	fn rlp_append(&self, s: &mut RlpStream);

	/// Get rlp-encoded bytes for this instance
	fn rlp_bytes(&self) -> Vec<u8> {
		let mut s = RlpStream::new();
		self.rlp_append(&mut s);
		s.drain()
	}
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        // TODO: check all transactions are the same too!
        self.index == other.index && self.timestamp == other.timestamp && self.hash == other.hash && self.previous_hash == other.previous_hash
    }
}

impl Block {
    fn genesis_block() -> Block {
    let timestamp = format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()); // TODO: handle unwrap properly here
        let hash = calculate_hash(&0, &String::new(), &timestamp, &String::new());
        Block {
            index: 0,
            timestamp: timestamp,
            data: vec!(),
            hash: hash,
            nonce: 0 as u64,
            previous_hash: String::new()
        }
    }
}

fn generate_next_block(block_data: UnverifiedTransaction, previous_block: &Block) -> Block {
    let timestamp = format!("{:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap()); // TODO: handle unwrap properly here
    let new_block_index = previous_block.index + 1;
    let hash = calculate_hash(&new_block_index, &previous_block.hash, &timestamp, block_data);
    Block {
        index: new_block_index,
        timestamp: timestamp,
        data: vec!(block_data),
        hash: hash,
        nonce: previous_block.nonce,
        previous_hash: previous_block.hash.clone()
    }
}

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
}

fn is_valid_new_block(new_block: &Block, previous_block: &Block) -> bool { // TODO: Make a result, which returns custom types
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

// TODO: make a trait instead for hashable? Move from bytes to type
fn calculate_hash(index: &u32, previous_hash: &str, timestamp: &str, data: &UnverifiedTransaction) -> String {
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
