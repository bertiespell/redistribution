// txIns unlock the coins and the txOuts ‘relock’ the coins
struct TxIn {
    pub txOutId: String,
    pub txOutIndex: usize,
    pub signature: String, // contains only the signature (created by the private-key), never the private-key itself
}