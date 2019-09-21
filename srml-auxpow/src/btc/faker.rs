use crate::auxpow::AuxPow;
use crate::btc::header::BtcHeader;
use crate::btc::transaction::BtcTx;
use crate::btc::transaction::OutPoint;
use crate::btc::transaction::TransactionInput;
use codec::Encode;
use primitives::H256;
use rstd::vec::Vec;

pub fn fake_coinbase(pre_hash: &H256) -> BtcTx {
    let merged_mining_magic = (0xfa, 0xbe, 0x6d, 0x6d);
    let merkle_size = 1;
    let merkle_nonce = 0;
    // 44 byte
    let script_sig = (merged_mining_magic, pre_hash, merkle_size, merkle_nonce).encode();
    let tx_input = TransactionInput {
        previous_output: OutPoint {
            hash: H256::default(),
            index: 0,
        },
        script_sig,
        sequence: 0,
    };

    BtcTx {
        version: 0,
        inputs: [tx_input].to_vec(),
        outputs: Vec::new(),
        lock_time: 0,
    }
}

pub fn fake_auxpow(pre_hash: &H256) -> AuxPow {
    let coinbase_txn = fake_coinbase(pre_hash);
    let parent_header = BtcHeader {
        version: 0x7fffffff,
        previous_header_hash: H256::default(),
        merkle_root: coinbase_txn.hash(),
        time: 0,
        bits: 0,  // do not care about parent block diff
        nonce: 0, // to be solved
    };

    AuxPow {
        coinbase_txn,
        parent_hash: parent_header.hash(),
        coinbase_branch: Vec::new(),
        coinbase_index: 0,
        blockchain_branch: Vec::new(),
        blockchain_index: 0,
        parent_header,
    }
}
