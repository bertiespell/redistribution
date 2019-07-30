use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{Error, ErrorKind, Result};
use std::time::{Duration, SystemTime};

use crate::encoder;
use crate::hasher;
use encoder::{Decodable, Encodable};
use hasher::calculate_hash;

pub type BlockData = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32, // height of the blockchain
    pub timestamp: Duration,
    pub data: BlockData,
    pub hash: String,
    pub previous_hash: String,
    pub difficulty: u32,
    pub nonce: u128,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
            && self.timestamp == other.timestamp
            && self.data == other.data
            && self.hash == other.hash
            && self.previous_hash == other.previous_hash
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: Duration,
        data: BlockData,
        hash: String,
        previous_hash: String,
        difficulty: u32,
        nonce: u128,
    ) -> Block {
        Block {
            index,
            timestamp,
            data,
            hash,
            previous_hash,
            difficulty,
            nonce,
        }
    }

    pub fn genesis_block() -> Block {
        let timestamp = Duration::new(0, 0);
        let difficulty: u32 = 0;
        let nonce: u128 = 0;
        let hash = calculate_hash(
            &0,
            &String::new(),
            &timestamp,
            &String::new(),
            &difficulty,
            &nonce,
        );
        Block::new(
            0,
            timestamp,
            String::new(),
            hash,
            String::new(),
            difficulty,
            nonce,
        )
    }

    pub fn calculate_hash_for_block(block: &Block) -> String {
        calculate_hash(
            &block.index,
            &block.previous_hash,
            &block.timestamp,
            &block.data,
            &block.difficulty,
            &block.nonce,
        )
    }
}

impl Encodable for Block {
    fn encode(&self) -> Result<Vec<u8>> {
        let serialized = serde_json::to_string(&self)?;
        Ok(serialized.into_bytes())
    }
}

impl Decodable for Block {
    fn decode(bytes: &Vec<u8>) -> Result<Self> {
        let json_string_result = String::from_utf8(bytes.clone());
        match json_string_result {
            Ok(json_string) => {
                let deserialized: Block = serde_json::from_str(&json_string)?;
                Ok(deserialized)
            }
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to decode Block - bytes not valid utf8",
            )),
        }
    }
}
