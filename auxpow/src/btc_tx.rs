use primitives::H256;
use codec::{Decode, Encode, IoReader};
use rustc_hex::{FromHex, FromHexError};
use std::{ str };
use crate::btc_hash::dhash256;

#[derive(Debug, Encode, Decode)]
pub struct Bytes(Vec<u8>);

impl From<&'static str> for Bytes {
	fn from(s: &'static str) -> Self {
		s.parse().unwrap()
	}
}

impl str::FromStr for Bytes {
	type Err = FromHexError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.from_hex().map(Bytes)
	}
}

#[derive(Debug, Encode, Decode)]
pub struct OutPoint {
	pub hash: H256,
	pub index: u32,
}

#[derive(Debug, Encode, Decode)]
pub struct TransactionInput {
	pub previous_output: OutPoint,
	pub script_sig: Bytes,
	pub sequence: u32,
	pub script_witness: Vec<Bytes>,
}

#[derive(Debug, Encode, Decode)]
pub struct TransactionOutput {
	pub value: u64,
	pub script_pubkey: Bytes,
}

#[derive(Debug, Encode, Decode)]
pub struct BtcTx {
	pub version: i32,
	pub inputs: Vec<TransactionInput>,
	pub outputs: Vec<TransactionOutput>,
	pub lock_time: u32,
}

impl BtcTx {
	fn hash(&self) -> H256 {
		dhash256(&self.encode())
	}
}

impl From<&'static str> for BtcTx {
	fn from(s: &'static str) -> Self {
		println!("1");
		let hex_data = &s.from_hex::<Vec<u8>>().unwrap();
		println!("2");
        let mut io_reader = IoReader(std::io::Cursor::new(hex_data as &[u8]));
		println!("3");
        Decode::decode(&mut io_reader).unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::{BtcTx, TransactionInput, OutPoint, TransactionOutput};
	use primitives::H256;
    use std::str::FromStr;

	// real transaction from block 80000
	// https://blockchain.info/rawtx/5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2
	// https://blockchain.info/rawtx/5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2?format=hex
	#[test]
	fn test_transaction_reader() {
		let t: BtcTx = "0100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".into();

	 	println!("{:#?}", t);
	
		// assert_eq!(t.version, 1);
		// assert_eq!(t.lock_time, 0);
		// assert_eq!(t.inputs.len(), 1);
		// assert_eq!(t.outputs.len(), 1);
		// let tx_input = &t.inputs[0];
		// assert_eq!(tx_input.sequence, 4294967295);
		// assert_eq!(tx_input.script_sig, "48304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501".into());
		// let tx_output = &t.outputs[0];
		// assert_eq!(tx_output.value, 5000000000);
		// assert_eq!(tx_output.script_pubkey, "76a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac".into());
	}

	// #[test]
	// fn test_transaction_hash() {
	// 	let t: BtcTx = "0100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".into();
	// 	let hash = H256::from_reversed_str("5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2");
	// 	assert_eq!(t.hash(), hash);
	// }

	// #[test]
	// fn test_transaction_reader_with_witness() {
	// 	// test case from https://github.com/bitcoin/bips/blob/master/bip-0143.mediawiki
	// 	let actual: BtcTx = "01000000000102fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f00000000494830450221008b9d1dc26ba6a9cb62127b02742fa9d754cd3bebf337f7a55d114c8e5cdd30be022040529b194ba3f9281a99f2b1c0a19c0489bc22ede944ccf4ecbab4cc618ef3ed01eeffffffef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a0100000000ffffffff02202cb206000000001976a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac9093510d000000001976a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac000247304402203609e17b84f6a7d30c80bfa610b5b4542f32a8a0d5447a12fb1366d7f01cc44a0220573a954c4518331561406f90300e8f3358f51928d43c212a8caed02de67eebee0121025476c2e83188368da1ff3e292e7acafcdb3566bb0ad253f62fc70f07aeee635711000000".into();
	// 	let expected = BtcTx {
	// 		version: 1,
	// 		inputs: vec![TransactionInput {
	// 			previous_output: OutPoint {
	// 				hash: "fff7f7881a8099afa6940d42d1e7f6362bec38171ea3edf433541db4e4ad969f".into(),
	// 				index: 0,
	// 			},
	// 			script_sig: "4830450221008b9d1dc26ba6a9cb62127b02742fa9d754cd3bebf337f7a55d114c8e5cdd30be022040529b194ba3f9281a99f2b1c0a19c0489bc22ede944ccf4ecbab4cc618ef3ed01".into(),
	// 			sequence: 0xffffffee,
	// 			script_witness: vec![],
	// 		}, TransactionInput {
	// 			previous_output: OutPoint {
	// 				hash: "ef51e1b804cc89d182d279655c3aa89e815b1b309fe287d9b2b55d57b90ec68a".into(),
	// 				index: 1,
	// 			},
	// 			script_sig: "".into(),
	// 			sequence: 0xffffffff,
	// 			script_witness: vec![
	// 				"304402203609e17b84f6a7d30c80bfa610b5b4542f32a8a0d5447a12fb1366d7f01cc44a0220573a954c4518331561406f90300e8f3358f51928d43c212a8caed02de67eebee01".into(),
	// 				"025476c2e83188368da1ff3e292e7acafcdb3566bb0ad253f62fc70f07aeee6357".into(),
	// 			],
	// 		}],
	// 		outputs: vec![TransactionOutput {
	// 			value: 0x0000000006b22c20,
	// 			script_pubkey: "76a9148280b37df378db99f66f85c95a783a76ac7a6d5988ac".into(),
	// 		}, TransactionOutput {
	// 			value: 0x000000000d519390,
	// 			script_pubkey: "76a9143bde42dbee7e4dbe6a21b2d50ce2f0167faa815988ac".into(),
	// 		}],
	// 		lock_time: 0x00000011,
	// 	};
	// 	assert_eq!(actual, expected);
	// }
}