pub mod btc_header;
pub mod btc_tx;
pub mod btc_hash;


use primitives::H256;
// use codec::{Decode, Encode};
use crate::btc_header::BtcHeader;
use crate::btc_tx::BtcTx;

// #[derive(Encode, Decode)]
pub struct AuxPow{
    /// Hash of the parent_header.
    pub parent_hash: H256,
    /// Parent block header.
    pub parent_header: BtcHeader,
    /// Coinbase transaction that is in the parent block, linking the AuxPOW 
    /// block to its parent block.
    pub coinbase_txn: BtcTx,
    /// The merkle branch linking the coinbase_txn to the parent block's 
    /// merkle_root.
    pub coinbase_branch: Vec<H256>,
    /// Bitmask of which side of the merkle hash function the coinbase_branch 
    /// element should go on. Zero means it goes on the right, One means on 
    /// the left. It is equal to the index of the starting hash within the 
    /// widest level of the merkle tree for this merkle branch.
    pub coinbase_index: u32,
    /// The merkle branch linking this auxiliary blockchain to the others, when 
    /// used in a merged mining setup with multiple auxiliary chains.
    pub blockchain_branch: Vec<H256>,
    /// Bitmask for blockchain_branch.
    pub blockchain_index: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auxpow_decode() {
        
    }
}
