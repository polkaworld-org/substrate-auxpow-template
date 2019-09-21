use crate::btc::hash::dhash256;
use codec::{Decode, Encode};
use primitives::H256;

#[derive(Debug, Encode, Decode)]
pub struct BtcHeader {
	pub version: u32,
	pub previous_header_hash: H256,
	pub merkle_root: H256,
	pub time: u32,
	pub bits: u32,
	pub nonce: u32,
}

impl BtcHeader {
	pub fn hash(&self) -> H256 {
		dhash256(&self.encode())
	}
}

#[cfg(test)]
mod tests {
	use super::BtcHeader;
	use codec::{Decode, IoReader};
	use primitives::H256;
	use rustc_hex::FromHex;
	use std::str::FromStr;

	impl From<&'static str> for BtcHeader {
		fn from(s: &'static str) -> Self {
			let hex_data = &s.from_hex::<Vec<u8>>().unwrap();
			let mut io_reader = IoReader(std::io::Cursor::new(hex_data as &[u8]));
			Decode::decode(&mut io_reader).unwrap()
		}
	}

	// Block 80000
	// https://blockchain.info/rawblock/000000000043a8c0fd1d6f726790caa2a406010d19efd2780db27bdbbd93baf6
	// https://blockchain.info/rawblock/000000000043a8c0fd1d6f726790caa2a406010d19efd2780db27bdbbd93baf6?format=hex
	#[test]
	fn test_block_merkle_root_and_hash() {
		let header: BtcHeader = "01000000ba8b9cda965dd8e536670f9ddec10e53aab14b20bacad27b9137190000000000190760b278fe7b8565fda3b968b918d5fd997f993b23674c0af3b6fde300b38f33a5914ce6ed5b1b01e32f570201000000010000000000000000000000000000000000000000000000000000000000000000ffffffff0704e6ed5b1b014effffffff0100f2052a01000000434104b68a50eaa0287eff855189f949c1c6e5f58b37c88231373d8a59809cbae83059cc6469d65c665ccfd1cfeb75c6e8e19413bba7fbff9bc762419a76d87b16086eac000000000100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".into();
		let merkle_root =
			H256::from_str("190760b278fe7b8565fda3b968b918d5fd997f993b23674c0af3b6fde300b38f")
				.unwrap();
		let hash =
			H256::from_str("f6ba93bddb7bb20d78d2ef190d0106a4a2ca9067726f1dfdc0a8430000000000")
				.unwrap();
		assert_eq!(header.merkle_root, merkle_root);
		assert_eq!(header.hash(), hash);

		// println!("{:#?}", header);
	}
}
