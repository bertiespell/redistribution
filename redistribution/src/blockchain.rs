use crate::encoder;
use crate::hasher;
use crate::Block;
use encoder::{Decodable, Encodable};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::VecDeque;
use std::io::{Error, ErrorKind, Result};
use std::time::SystemTime;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Blockchain {
    blocks: VecDeque<Block>,
}

impl Blockchain {
    pub fn new() -> Result<Blockchain> {
        let genesis_block = Block::genesis_block()?;
        let mut blocks = VecDeque::new();
        blocks.push_back(genesis_block);

        Ok(Blockchain { blocks })
    }

    pub fn add_block(&mut self, block: Block) -> Result<()> {
        let last_block_result = self.blocks.back();
        match last_block_result {
            Some(last_block) => {
                if Blockchain::is_valid_new_block(&block, last_block) {
                    Ok(self.blocks.push_back(block))
                } else {
                    Err(Error::new(ErrorKind::InvalidData, "Invalid block"))
                }
            }
            None => Err(Error::new(
                ErrorKind::InvalidData,
                "No last block to append to",
            )),
        }
    }

    pub fn generate_next_block(&self, block_data: &str) -> Result<Block> {
        let now_result = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
        match now_result {
            Ok(now) => {
                let timestamp = format!("{:?}", now);
                let previous_block = self.get_latest_block()?;
                let new_block_index = previous_block.index + 1;
                let difficulty: u32 = 0;
                let nonce: u128 = 0;
                let hash = hasher::calculate_hash(
                    &new_block_index,
                    &previous_block.hash,
                    &timestamp,
                    block_data,
                    &difficulty,
                    &nonce,
                );
                Ok(Block::new(
                    new_block_index,
                    timestamp,
                    block_data.to_string(),
                    hash,
                    previous_block.hash.clone(),
                    difficulty,
                    nonce,
                ))
            }
            Err(_) => Err(Error::new(
                ErrorKind::NotFound,
                "Error reporting system time",
            )),
        }
    }

    fn get_latest_block(&self) -> Result<&Block> {
        let last_block_result = self.blocks.back();
        match last_block_result {
            Some(block) => Ok(block),
            None => Err(Error::new(
                ErrorKind::NotFound,
                "Unable to locate last block",
            )),
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

    pub fn is_chain_valid(blockchain: &Blockchain) -> bool {
        // TODO: need to check genesis block somehow
        blockchain
            .blocks
            .iter()
            .skip(1)
            .zip(blockchain.blocks.iter())
            .map(|(block, last_block)| block.previous_hash == last_block.hash)
            .fold(true, |x, y| x && y)
    }

    pub fn determine_longest_chain<'a>(
        first_blockchain: &'a Blockchain,
        second_blockchain: &'a Blockchain,
    ) -> Result<&'a Blockchain> {
        let last_block_in_first_chain = first_blockchain.blocks.back();
        match last_block_in_first_chain {
            Some(first_block) => {
                let last_block_in_second_chain = second_blockchain.blocks.back();
                match last_block_in_second_chain {
                    Some(second_block) => {
                        if first_block.index > second_block.index {
                            return Ok(first_blockchain);
                        }
                        Ok(second_blockchain)
                    }
                    None => Err(Error::new(
                        ErrorKind::NotFound,
                        "Unable to locate last block in second chain",
                    )),
                }
            }
            None => Err(Error::new(
                ErrorKind::NotFound,
                "Unable to locate last block in first chain",
            )),
        }
    }
}

impl Encodable for Blockchain {
    fn encode(&self) -> Result<Vec<u8>> {
        let serialized = serde_json::to_string(&self)?;
        Ok(serialized.into_bytes())
    }
}

impl Decodable for Blockchain {
    fn decode(bytes: &Vec<u8>) -> Result<Self> {
        let json_string_result = String::from_utf8(bytes.clone());
        match json_string_result {
            Ok(json_string) => {
                let deserialized: Blockchain = serde_json::from_str(&json_string)?;
                Ok(deserialized)
            }
            Err(_) => Err(Error::new(
                ErrorKind::InvalidData,
                "Unable to decode Blockchain - bytes not valid utf8",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block_validity() {
        let blockchain = Blockchain::new().unwrap();
        let genesis_block = blockchain.get_latest_block().unwrap();
        let next_block = blockchain.generate_next_block("Test block data!").unwrap();
        let block_is_valid = Blockchain::is_valid_new_block(&next_block, &genesis_block);
        assert_eq!(block_is_valid, true);
    }

    #[test]
    fn test_chain_validity() {
        let mut blockchain = Blockchain::new().unwrap();
        let new_block1 = blockchain.generate_next_block("Block 1");
        blockchain.add_block(new_block1.unwrap());
        let new_block2 = blockchain.generate_next_block("Block 2");
        blockchain.add_block(new_block2.unwrap());
        let new_block3 = blockchain.generate_next_block("Block 3");
        blockchain.add_block(new_block3.unwrap());

        let validity = Blockchain::is_chain_valid(&blockchain);
        assert_eq!(validity, true);
    }
}
