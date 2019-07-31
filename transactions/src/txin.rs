// txIns unlock the coins and the txOuts ‘relock’ the coins
pub struct TxIn {
    pub tx_out_id: String,
    pub tx_out_index: usize,
    pub signature: String, // contains only the signature (created by the private-key), never the private-key itself
}