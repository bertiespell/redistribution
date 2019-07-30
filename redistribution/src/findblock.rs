use crate::block;
use crate::difficulty;
use crate::hasher;
use block::{Block, BlockData};
use difficulty::hash_matches_difficulty;
use hasher::calculate_hash;
use std::time::Duration;

fn find_block(
    index: u32,
    previous_hash: String,
    timestamp: Duration,
    data: BlockData,
    difficulty: u32,
) -> Block {
    let mut nonce: u128 = 0;

    loop {
        let hash: String = calculate_hash(
            &index,
            &previous_hash,
            &timestamp,
            &data,
            &difficulty,
            &nonce,
        );

        if hash_matches_difficulty(&hash, &difficulty).unwrap() {
            break Block {
                index,
                timestamp,
                data,
                hash,
                previous_hash,
                difficulty,
                nonce,
            };
        }
        nonce += 1;
    }
}
