use openssl::sha;
use std::time::Duration;

pub fn calculate_hash(
    index: &u32,
    previous_hash: &str,
    timestamp: &Duration,
    data: &str,
    difficulty: &u32,
    nonce: &u128,
) -> String {
    let mut hasher = sha::Sha256::new();
    hasher.update(&index.to_be_bytes());
    hasher.update(&previous_hash.as_bytes());
    hasher.update(&timestamp.as_micros().to_le_bytes());
    hasher.update(&data.as_bytes());
    hasher.update(&difficulty.to_be_bytes());
    hasher.update(&nonce.to_be_bytes());

    let hash = hasher.finish();
    hex::encode(hash)
}
