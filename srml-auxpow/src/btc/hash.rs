extern crate hmac_sha256;

use hmac_sha256::Hash;
use primitives::H256;

/// SHA-256
pub fn sha256(input: &[u8]) -> H256 {
	let mut h = Hash::new();
	h.update(input);
	let result = h.finalize();
	H256::from_slice(&result[..])
}

/// Double SHA-256
pub fn dhash256(data: &[u8]) -> H256 {
	let first = sha256(data);
	let second = sha256(first.as_ref());
	second
}
