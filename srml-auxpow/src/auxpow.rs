use crate::btc::header::BtcHeader;
use crate::btc::transaction::BtcTx;
use codec::{Decode, Encode, Error, Input, Output};
use primitives::H256;
use rstd::vec::Vec;

#[derive(Debug)]
pub struct AuxPow {
    /// Coinbase transaction that is in the parent block, linking the AuxPOW
    /// block to its parent block.
    pub coinbase_txn: BtcTx,
    /// Hash of the parent_header.
    pub parent_hash: H256,
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
    /// Parent block header.
    pub parent_header: BtcHeader,
}

impl AuxPow {
    pub fn verify(&self, pre_hash: &H256) -> bool {
        // todo
        // check coinbase merkle branch
        // check blockchain merkle branch
        // check merged mining magic number, fabe6d6d
        // check chainID
        true
    }
}

impl Encode for AuxPow {
    fn encode_to<T: Output>(&self, dest: &mut T) {
        self.coinbase_txn.encode_to(dest);
        self.parent_hash.encode_to(dest);
        (self.coinbase_branch.len() as u8).encode_to(dest);
        for i in 0..self.coinbase_branch.len() {
            self.coinbase_branch[i].encode_to(dest);
        }
        self.coinbase_index.encode_to(dest);
        (self.blockchain_branch.len() as u8).encode_to(dest);
        for i in 0..self.blockchain_branch.len() {
            self.blockchain_branch[i].encode_to(dest);
        }
        self.blockchain_index.encode_to(dest);
        self.parent_header.encode_to(dest);
    }
}

impl Decode for AuxPow {
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        let coinbase_txn: BtcTx = Decode::decode(input)?;
        let parent_hash: H256 = Decode::decode(input)?;
        let len: u8 = Decode::decode(input)?;
        let mut coinbase_branch: Vec<H256> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            coinbase_branch.push(Decode::decode(input)?);
        }
        let coinbase_index: u32 = Decode::decode(input)?;

        let len: u8 = Decode::decode(input)?;
        let mut blockchain_branch: Vec<H256> = Vec::with_capacity(len as usize);
        for _ in 0..len {
            blockchain_branch.push(Decode::decode(input)?);
        }
        let blockchain_index: u32 = Decode::decode(input)?;
        let parent_header: BtcHeader = Decode::decode(input)?;

        Ok(AuxPow {
            coinbase_txn,
            parent_hash,
            coinbase_branch,
            coinbase_index,
            blockchain_branch,
            blockchain_index,
            parent_header,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use codec::IoReader;
    use rustc_hex::FromHex;

    impl From<&'static str> for AuxPow {
        fn from(s: &'static str) -> Self {
            let hex_data = &s.from_hex::<Vec<u8>>().unwrap();
            let mut io_reader = IoReader(std::io::Cursor::new(hex_data as &[u8]));
            Decode::decode(&mut io_reader).unwrap()
        }
    }
    #[test]
    fn auxpow_decode() {
        let auxpow: AuxPow = "02000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4b039aff0904db044a5b742f4254432e434f4d2ffabe6d6d35ecfc5f5ca2971449ee78b7d810f280de7e3e7c407e3c0162ef8692df350ef8020000004204cb9a011fde202e00000000000000ffffffff0200000000000000001976a914c0174e89bd93eacd1d5a1af4ba1802d412afc08688ac0000000000000000266a24aa21a9ede2f61c3f71d1defd3fa999dfa36953755c690689799962b48bebd836974e8cf9000000001d1879510258c5186e39cfcde4539c88686854b1ca640681dd38ed9527e635600000000000015f2f03802d61504f12e25d4b679b881ddb374cc04f240b6eb765d887679fb6360000000000000020a9f32bdb09d7777f3fa308fcd221e531393441f50e7f8b2d4ef63b2c3440940ec866338e7674b07d6a92269317f09f6c0fdb60ce7052e0211133e0015727ebb2db044a5bffff7f20db044a5b".into();
        // println!("{:#?}", auxpow);
    }
}
