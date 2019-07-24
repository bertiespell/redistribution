use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use serde_json;
use std::io::{Result, Error, ErrorKind};

use crate::encoder;
use encoder::{Encodable, Decodable};
use crate::hasher;
use hasher::calculate_hash;

pub type BlockData = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32, // height of the blockchain
    pub timestamp: String,
    pub data: BlockData,
    pub hash: String,
    pub previous_hash: String,
    pub difficuly: u128,
    pub nonce: u32,
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.timestamp == other.timestamp && self.data == other.data && self.hash == other.hash && self.previous_hash == other.previous_hash
    }
}

impl Block {
    pub fn new(index: u32, timestamp: String, data: BlockData, hash: String, previous_hash: String, difficuly: u128, nonce: u32) -> Block {
        Block {
            index,
            timestamp,
            data,
            hash,
            previous_hash,
            difficuly,
            nonce,
        }
    }

    pub fn genesis_block() -> Result<Block> {
        let system_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH); // TODO: Don't need system time here
        match system_time {
            Ok(time) => {
                let timestamp = format!("{:?}", time); // TODO: genesis block should just have a set timestamp
                let hash = calculate_hash(&0, &String::new(), &timestamp, &String::new());
                let difficulty: u128 = 0;
                let nonce: u32 = 0;
                Ok(Block::new(0, timestamp, String::new(), hash, String::new(), difficulty, nonce))
            },
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Error getting system time"))
        }
    }

    pub fn calculate_hash_for_block(block: &Block) -> String {
        calculate_hash(&block.index, &block.previous_hash, &block.timestamp, &block.data)
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
            },
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Unable to decode Block - bytes not valid utf8"))
        }
    }
}