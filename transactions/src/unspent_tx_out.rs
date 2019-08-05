use crate::sign_transaction::find_unspent_txout;
use crate::transaction::Transaction;

#[derive(Clone)]
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

pub fn update_unspent_tx_outs(
    new_transactions: Vec<Transaction>,
    all_unspent_tx_outs: Vec<UnspentTxOut>,
) -> Vec<UnspentTxOut> {
    let new_unspent_tx_out: Vec<UnspentTxOut> = new_transactions
        .iter()
        .map(|transaction| {
            transaction
                .tx_outs
                .iter()
                .enumerate()
                .map(|(index, tx_out)| {
                    UnspentTxOut::new(
                        transaction.id.clone(),
                        index,
                        tx_out.address.clone(),
                        tx_out.amount,
                    )
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<UnspentTxOut>>();

    let consumed_tx_outs: Vec<UnspentTxOut> = new_transactions
        .iter()
        .map(|transaction| transaction.tx_ins.clone())
        .flatten()
        .map(|tx_in| UnspentTxOut::new(tx_in.tx_out_id, tx_in.tx_out_index, String::new(), 0))
        .collect();

    let mut resulting_unspent_tx_outs = all_unspent_tx_outs
        .iter()
        .filter_map(|unspent_transaction| {
            find_unspent_txout(
                &unspent_transaction.tx_out_id,
                unspent_transaction.tx_out_index,
                consumed_tx_outs.clone(),
            )
        })
        .collect::<Vec<UnspentTxOut>>();
    resulting_unspent_tx_outs.extend_from_slice(&new_unspent_tx_out[..]);
    resulting_unspent_tx_outs
}
