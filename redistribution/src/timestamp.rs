use crate::block::Block;
/**
 * const isValidTimestamp = (newBlock: Block, previousBlock: Block): boolean => {
    return ( previousBlock.timestamp - 60 < newBlock.timestamp )
        && newBlock.timestamp - 60 < getCurrentTimestamp();
};
 */
use std::io::{Error, ErrorKind, Result};
use std::time::{Duration, SystemTime};

fn is_valid_timestamp(new_block: Block, previous_block: Block) -> Result<bool> {
    Ok(
        previous_block.timestamp - Duration::new(60, 0) < new_block.timestamp
            && new_block.timestamp - Duration::new(60, 0) < get_current_timestamp()?,
    )
}

pub fn get_current_timestamp() -> Result<Duration> {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(time) => Ok(time),
        Err(_) => Err(Error::new(
            ErrorKind::NotFound,
            "Error reporting system time",
        )),
    }
}
