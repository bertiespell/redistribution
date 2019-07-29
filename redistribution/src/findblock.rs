use crate::block;
use crate::hasher;
use crate::difficulty;
use block::{Block, BlockData};
use hasher::calculate_hash;
use difficulty::hash_matches_difficulty;

fn find_block(index: u32, previous_hash: String, timestamp: String, data: BlockData, difficulty: u32) -> Block {
    let mut nonce: u128 = 0;

    loop {
        let hash: String = calculate_hash(&index, &previous_hash, &timestamp, &data, &difficulty, &nonce);

        if hash_matches_difficulty(&hash, &difficulty).unwrap() {
            break Block {
                index,
                timestamp,
                data,
                hash,
                previous_hash,
                difficulty,
                nonce
            }
        }
        nonce += 1;
    }
}