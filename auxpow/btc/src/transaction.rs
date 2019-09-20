use crate::hash::dhash256;
use codec::{Decode, Encode, Error, Input, Output};
use primitives::H256;
use std::str;

#[derive(Debug, Encode, Decode)]
pub struct OutPoint {
	pub hash: H256,
	pub index: u32,
}

#[derive(Debug)]
pub struct TransactionInput {
	pub previous_output: OutPoint,
	pub script_sig: Vec<u8>,
	pub sequence: u32,
}

impl Encode for TransactionInput {
	fn encode_to<T: Output>(&self, dest: &mut T) {
		self.previous_output.encode_to(dest);
		(self.script_sig.len() as u8).encode_to(dest);
		for i in 0..self.script_sig.len() {
			self.script_sig[i].encode_to(dest);
		}

		self.sequence.encode_to(dest);
	}
}

impl Decode for TransactionInput {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		let previous_output: OutPoint = Decode::decode(input)?;
		let len: u8 = Decode::decode(input)?;
		let mut script_sig: Vec<u8> = Vec::with_capacity(len as usize);
		for _ in 0..len {
			script_sig.push(Decode::decode(input)?);
		}
		let sequence: u32 = Decode::decode(input)?;

		Ok(TransactionInput {
			previous_output,
			script_sig,
			sequence,
		})
	}
}

#[derive(Debug)]
pub struct TransactionOutput {
	pub value: u64,
	pub script_pubkey: Vec<u8>,
}

impl Encode for TransactionOutput {
	fn encode_to<T: Output>(&self, dest: &mut T) {
		self.value.encode_to(dest);
		(self.script_pubkey.len() as u8).encode_to(dest);
		for i in 0..self.script_pubkey.len() {
			self.script_pubkey[i].encode_to(dest);
		}
	}
}

impl Decode for TransactionOutput {
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		let value: u64 = Decode::decode(input)?;
		let len: u8 = Decode::decode(input)?;
		let mut script_pubkey: Vec<u8> = Vec::with_capacity(len as usize);
		for _ in 0..len {
			script_pubkey.push(Decode::decode(input)?);
		}

		Ok(TransactionOutput {
			value,
			script_pubkey,
		})
	}
}

#[derive(Debug)]
pub struct BtcTx {
	pub version: i32,
	pub inputs: Vec<TransactionInput>,
	pub outputs: Vec<TransactionOutput>,
	pub lock_time: u32,
}

impl Encode for BtcTx {
	fn encode_to<T: Output>(&self, dest: &mut T) {
		self.version.encode_to(dest);
		(self.inputs.len() as u8).encode_to(dest);
		for i in 0..self.inputs.len() {
			self.inputs[i].encode_to(dest);
		}
		(self.outputs.len() as u8).encode_to(dest);
		for i in 0..self.outputs.len() {
			self.outputs[i].encode_to(dest);
		}
		self.lock_time.encode_to(dest);
	}
}

impl Decode for BtcTx {
	fn decode<I: Input>(src: &mut I) -> Result<Self, Error> {
		let version: i32 = Decode::decode(src)?;
		let input_len: u8 = Decode::decode(src)?;
		let mut inputs: Vec<TransactionInput> = Vec::with_capacity(input_len as usize);
		for _ in 0..input_len {
			inputs.push(Decode::decode(src)?);
		}
		let output_len: u8 = Decode::decode(src)?;
		let mut outputs: Vec<TransactionOutput> = Vec::with_capacity(output_len as usize);
		for _ in 0..output_len {
			outputs.push(Decode::decode(src)?);
		}
		let lock_time: u32 = Decode::decode(src)?;

		Ok(BtcTx {
			version,
			inputs,
			outputs,
			lock_time,
		})
	}
}

impl BtcTx {
	fn hash(&self) -> H256 {
		dhash256(&self.encode())
	}
}

#[cfg(test)]
mod tests {
	use super::BtcTx;
	use codec::{Decode, IoReader};
	use primitives::H256;
	use rustc_hex::FromHex;
	use std::str::FromStr;

	impl From<&'static str> for BtcTx {
		fn from(s: &'static str) -> Self {
			let hex_data = &s.from_hex::<Vec<u8>>().unwrap();
			let mut io_reader = IoReader(std::io::Cursor::new(hex_data as &[u8]));
			Decode::decode(&mut io_reader).unwrap()
		}
	}

	// real transaction from block 80000
	// https://blockchain.info/rawtx/5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2
	// https://blockchain.info/rawtx/5a4ebf66822b0b2d56bd9dc64ece0bc38ee7844a23ff1d7320a88c5fdb2ad3e2?format=hex
	#[test]
	fn test_transaction_reader() {
		let t: BtcTx = "0100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".into();
		assert_eq!(t.version, 1);
		assert_eq!(t.lock_time, 0);
		assert_eq!(t.inputs.len(), 1);
		assert_eq!(t.outputs.len(), 1);
		let tx_input = &t.inputs[0];
		assert_eq!(tx_input.sequence, 4294967295);
		let tx_output = &t.outputs[0];
		assert_eq!(tx_output.value, 5000000000);

		// println!("{:?}", t);
	}

	#[test]
	fn test_transaction_hash() {
		let t: BtcTx = "0100000001a6b97044d03da79c005b20ea9c0e1a6d9dc12d9f7b91a5911c9030a439eed8f5000000004948304502206e21798a42fae0e854281abd38bacd1aeed3ee3738d9e1446618c4571d1090db022100e2ac980643b0b82c0e88ffdfec6b64e3e6ba35e7ba5fdd7d5d6cc8d25c6b241501ffffffff0100f2052a010000001976a914404371705fa9bd789a2fcd52d2c580b65d35549d88ac00000000".into();
		let hash =
			H256::from_str("e2d32adb5f8ca820731dff234a84e78ec30bce4ec69dbd562d0b2b8266bf4e5a")
				.unwrap();
		assert_eq!(t.hash(), hash);
	}
}
