// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

use crate::auxpow::AuxPow;
use crate::btc::faker::fake_auxpow;
use codec::{Decode, Encode};
use pow_primitives::{Difficulty, Seal};
use primitives::{H256, U256};
use runtime_io::print;

fn verify_difficulty(hash: &H256, difficulty: Difficulty, proportion: Difficulty) -> bool {
	let target = U256::from(difficulty) * U256::from(proportion);
	let seal = U256::from(&hash[..]);

	seal > target
}

pub fn verify(
	pre_hash: &H256,
	seal: &Seal,
	difficulty: Difficulty,
	proportion: Difficulty,
) -> bool {
	print("begin auxpow verify!");
	let auxpow = match AuxPow::decode(&mut &seal[..]) {
		Ok(seal) => seal,
		Err(_) => return false,
	};

	if !auxpow.verify(pre_hash) {
		return false;
	}

	if !verify_difficulty(&auxpow.parent_hash, difficulty, proportion) {
		return false;
	}

	true
}

pub fn mine(
	pre_hash: &H256,
	_seed: &H256,
	difficulty: Difficulty,
	round: u32,
	proportion: Difficulty,
) -> Option<Seal> {
	for i in 0..round {
		print("begin auxpow mining!");

		// Make a fake auxpow for CPU mining purpose
		let mut auxpow = fake_auxpow(pre_hash);
		auxpow.parent_header.nonce = i;
		let work = auxpow.parent_header.hash();

		if verify_difficulty(&work, difficulty, proportion) {
			auxpow.parent_hash = work;
			return Some(auxpow.encode());
		}
	}

	None
}
