use std::io::{Result, Error, ErrorKind};

pub fn hash_matches_difficulty(hash: &String, difficulty: &u32) -> Result<bool> {
    dbg!(hash.as_bytes());
    let decoded_hex_result = hex::decode(hash);
    match decoded_hex_result {
        Ok(decoded_hex) => {
            let mut last_found = true;
            let mut once = true;
            let leading_zeros = decoded_hex.iter().map(|x| x.leading_zeros()).fold(0, |acc, x| {
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
        },
        Err(_) => Err(Error::new(ErrorKind::InvalidData, "Could not decode hex from hash"))
    }
} 

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
