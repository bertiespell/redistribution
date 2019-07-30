use std::io::{Error, ErrorKind, Result};

pub fn hash_matches_difficulty(hash: &String, difficulty: &u32) -> Result<bool> {
    dbg!(hash.as_bytes());
    let decoded_hex_result = hex::decode(hash);
    match decoded_hex_result {
        Ok(decoded_hex) => {
            let mut last_found = true;
            let mut once = true;
            let leading_zeros = decoded_hex
                .iter()
                .map(|x| x.leading_zeros())
                .fold(0, |acc, x| {
                    if x == 8 && last_found {
                        return acc + x;
                    } else if once {
                        last_found = false;
                        once = false;
                        return acc + x;
                    } else {
                        return acc;
                    }
                });
            Ok(leading_zeros >= *difficulty)
        }
        Err(_) => Err(Error::new(
            ErrorKind::InvalidData,
            "Could not decode hex from hash",
        )),
    }
}

// in seconds
const BLOCK_GENERATION_INTERVAL: u32 = 10;

// in blocks
const DIFFICULTY_ADJUSTMENT_INTERVAL: u32 = 10;

use crate::blockchain;
use crate::block;


fn get_difficulty(blockchain: blockchain::Blockchain) -> u32 {
    let latest_block = blockchain.get_latest_block().unwrap();
    if latest_block.index % DIFFICULTY_ADJUSTMENT_INTERVAL == 0 && latest_block.index != 0 {
        return get_adjusted_difficulty(latest_block, &blockchain);
    } else {
        return latest_block.difficulty;
    }
}

fn get_adjusted_difficulty(latest_block: &block::Block, chain: &blockchain::Blockchain) -> u32 {
    let previous_adjustment_block = chain.get_block_at_index(chain.len() - DIFFICULTY_ADJUSTMENT_INTERVAL as usize).unwrap(); // TODO: handle
    let time_expected = BLOCK_GENERATION_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL;
    let time_taken = latest_block.timestamp - previous_adjustment_block.timestamp;

    3
}
/**
const getAdjustedDifficulty = (latestBlock: Block, aBlockchain: Block[]) => {
    const prevAdjustmentBlock: Block = aBlockchain[blockchain.length - DIFFICULTY_ADJUSTMENT_INTERVAL];
    const timeExpected: number = BLOCK_GENERATION_INTERVAL * DIFFICULTY_ADJUSTMENT_INTERVAL;
    const timeTaken: number = latestBlock.timestamp - prevAdjustmentBlock.timestamp;
    if (timeTaken < timeExpected / 2) {
        return prevAdjustmentBlock.difficulty + 1;
    } else if (timeTaken > timeExpected * 2) {
        return prevAdjustmentBlock.difficulty - 1;
    } else {
        return prevAdjustmentBlock.difficulty;
    }
};
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_difficulty() {
        let test_case = hex::encode("ABCABCABC");
        let matches = hash_matches_difficulty(&test_case, &1).unwrap();
        assert_eq!(matches, true);
        let test_case = hex::encode("11BCABCABC");
        let matches = hash_matches_difficulty(&test_case, &2).unwrap();
        assert_eq!(matches, true);
    }
}
