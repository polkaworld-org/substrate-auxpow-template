extern crate crypto as rcrypto;

pub use rcrypto::digest::Digest;
use rcrypto::sha2::Sha256;
use primitives::H256;

/// SHA-256
#[inline]
pub fn sha256(input: &[u8]) -> H256 {
	let mut result: [u8; 32] = [0; 32];
	let mut hasher = Sha256::new();
	hasher.input(input);
	hasher.result(&mut result);
	H256::from_slice(&result)
}

/// Double SHA-256
#[inline]
pub fn dhash256(input: &[u8]) -> H256 {
	let first = sha256(input);
	let second = sha256(first.as_ref());
	second
}
