// txIns unlock the coins and the txOuts ‘relock’ the coins
struct TxOut {
    pub address: String,
    pub amount: usize,
}

impl TxOut {
    fn new(address: String, amount: usize) -> TxOut {
        TxOut {
            address,
            amount
        }
    }
}