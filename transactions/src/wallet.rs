use crate::unspent_tx_out::UnspentTxOut;

pub struct Wallet {
    pub unspent_tx_outs: Vec<UnspentTxOut>,
}
