pub struct UnspentTxOut {
    pub tx_out_id: String,
    pub tx_out_index: usize,
    pub address: String,
    pub amount: usize,
}

impl UnspentTxOut {
    pub fn new(
        tx_out_id: String,
        tx_out_index: usize,
        address: String,
        amount: usize,
    ) -> UnspentTxOut {
        UnspentTxOut {
            tx_out_id,
            tx_out_index,
            address,
            amount,
        }
    }
}
