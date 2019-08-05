use crate::txin;
use crate::txout;
use openssl::sha;

pub struct Transaction {
    pub id: String,
    pub tx_ins: Vec<txin::TxIn>,
    pub tx_outs: Vec<txout::TxOut>,
}

/**
The transaction id is calculated by taking a hash from the contents of the transaction. However, the signatures of the txIds are not included in the transaction hash as the will be added later on to the transaction.
 */

fn get_transaction_id(transaction: Transaction) -> String {
    let txin_content = transaction
        .tx_ins
        .iter()
        .map(|txin| {
            let mut bytes: Vec<u8> = vec![];
            txin.tx_out_id
                .as_bytes()
                .iter()
                .for_each(|byte| bytes.push(*byte));
            txin.tx_out_index
                .to_le_bytes()
                .iter()
                .for_each(|byte| bytes.push(*byte));
            bytes
        })
        .flatten()
        .collect::<Vec<u8>>();

    let txout_content = transaction
        .tx_outs
        .iter()
        .map(|txout| {
            let mut bytes: Vec<u8> = vec![];
            txout
                .address
                .as_bytes()
                .iter()
                .for_each(|byte| bytes.push(*byte));
            txout
                .amount
                .to_le_bytes()
                .iter()
                .for_each(|byte| bytes.push(*byte));
            bytes
        })
        .flatten()
        .collect::<Vec<u8>>();

    calculate_hash(&txin_content, &txout_content)
}

pub fn calculate_hash(txin_content: &[u8], txout_content: &[u8]) -> String {
    let mut hasher = sha::Sha256::new();
    hasher.update(&txin_content);
    hasher.update(&txout_content);
    let hash = hasher.finish();
    hex::encode(hash)
}
