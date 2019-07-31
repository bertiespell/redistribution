pub mod sign_transaction;
pub mod transaction;
pub mod txin;
pub mod txout;
pub mod unspent_tx_out;
pub mod wallet;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
