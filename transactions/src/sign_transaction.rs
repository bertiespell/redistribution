use crate::sign_transaction;
use crate::transaction::Transaction;
use crate::txin;
use crate::unspent_tx_out::UnspentTxOut;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey, SerializedSignature};
use std::io::{Error, ErrorKind, Result};
use std::ops::Index;

fn sign_tx_in(
    transaction: Transaction,
    txin_index: usize,
    private_key: SecretKey,
    unspent_tx_outs: Vec<UnspentTxOut>,
) -> Result<SerializedSignature> {
    let tx_in: &txin::TxIn = transaction.txIns.index(txin_index);
    let data_to_sign = Message::from_slice(transaction.id.as_bytes()).unwrap();

    match find_unspent_txout(&tx_in.tx_out_id, tx_in.tx_out_index, unspent_tx_outs) {
        Some(referenced_unspect_txout) => {
            let referencedAddress = referenced_unspect_txout.address;
            let secp = Secp256k1::new();
            let public_key = PublicKey::from_secret_key(&secp, &private_key);

            let sig = secp.sign(&data_to_sign, &private_key);
            Ok(sig.serialize_der())
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Could not find unspent txs out",
        )),
    }
}

fn find_unspent_txout(
    transaction_id: &str,
    out_index: usize,
    unspent_tx_outs: Vec<sign_transaction::UnspentTxOut>,
) -> Option<UnspentTxOut> {
    unspent_tx_outs
        .into_iter()
        .find(|uTxO| uTxO.tx_out_id == transaction_id && uTxO.tx_out_index == out_index)
}
