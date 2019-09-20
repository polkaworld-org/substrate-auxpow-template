extern crate sha2;

use primitives::H256;
use sha2::{Digest, Sha256};

/// SHA-256
pub fn sha256(data: &[u8]) -> H256 {
	let mut hasher = Sha256::new();
	hasher.input(data);
	let result = hasher.result();
	H256::from_slice(&result[..])
}

/// Double SHA-256
#[inline]
pub fn dhash256(data: &[u8]) -> H256 {
	let first = sha256(data);
	let second = sha256(first.as_ref());
	second
}
