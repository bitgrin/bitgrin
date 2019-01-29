// Copyright 2018 The Grin Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Definition of the genesis block. Placeholder for now.

// required for genesis replacement
//! #![allow(unused_imports)]

use chrono::prelude::{TimeZone, Utc};

use crate::core;
use crate::global;
use crate::pow::{Difficulty, Proof, ProofOfWork};
use crate::util;
use crate::util::secp::constants::SINGLE_BULLET_PROOF_SIZE;
use crate::util::secp::pedersen::{Commitment, RangeProof};
use crate::util::secp::Signature;

use crate::core::hash::Hash;
use crate::keychain::BlindingFactor;

/// Genesis block definition for development networks. The proof of work size
/// is small enough to mine it on the fly, so it does not contain its own
/// proof of work solution. Can also be easily mutated for different tests.
pub fn genesis_dev() -> core::Block {
	core::Block::with_header(core::BlockHeader {
		height: 0,
		// previous: core::hash::Hash([0xff; 32]),
		timestamp: Utc.ymd(1997, 8, 4).and_hms(0, 0, 0),
		pow: ProofOfWork {
			nonce: global::get_genesis_nonce(),
			..Default::default()
		},
		..Default::default()
	})
}

/// Placeholder for floonet genesis block, will definitely change before
/// release
pub fn genesis_floo() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader {
		height: 0,
		timestamp: Utc.ymd(2018, 12, 28).and_hms(20, 48, 4),
		prev_root: Hash::from_hex(
			"00000000000000000017ff4903ef366c8f62e3151ba74e41b8332a126542f538",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"73b5e0a05ea9e1e4e33b8f1c723bc5c10d17f07042c2af7644f4dbb61f4bc556",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"667a3ba22f237a875f67c9933037c8564097fa57a3e75be507916de28fc0da26",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"cfdddfe2d938d0026f8b1304442655bbdddde175ff45ddf44cb03bcb0071a72d",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(10_u64.pow(5)),
			secondary_scaling: 1856,
			nonce: 23,
			proof: Proof {
				nonces: vec![
					16994232, 22975978, 32664019, 44016212, 50238216, 57272481, 85779161,
					124272202, 125203242, 133907662, 140522149, 145870823, 147481297, 164952795,
					177186722, 183382201, 197418356, 211393794, 239282197, 239323031, 250757611,
					281414565, 305112109, 308151499, 357235186, 374041407, 389924708, 390768911,
					401322239, 401886855, 406986280, 416797005, 418935317, 429007407, 439527429,
					484809502, 486257104, 495589543, 495892390, 525019296, 529899691, 531685572,
				],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
		excess: Commitment::from_vec(
			util::from_hex(
				"08df2f1d996cee37715d9ac0a0f3b13aae508d1101945acb8044954aee30960be9".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[
			25, 176, 52, 246, 172, 1, 12, 220, 247, 111, 73, 101, 13, 16, 157, 130, 110, 196, 123,
			217, 246, 137, 45, 110, 106, 186, 0, 151, 255, 193, 233, 178, 103, 26, 210, 215, 200,
			89, 146, 188, 9, 161, 28, 212, 227, 143, 82, 54, 5, 223, 16, 65, 237, 132, 196, 241,
			39, 76, 133, 45, 252, 131, 88, 0,
		])
		.unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(
				"08c12007af16d1ee55fffe92cef808c77e318dae70c3bc70cb6361f49d517f1b68".to_string(),
			)
			.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [
				159, 156, 202, 179, 128, 169, 14, 227, 176, 79, 118, 180, 62, 164, 2, 234, 123, 30,
				77, 126, 232, 124, 42, 186, 239, 208, 21, 217, 228, 246, 148, 74, 100, 25, 247,
				251, 82, 100, 37, 16, 146, 122, 164, 5, 2, 165, 212, 192, 221, 167, 199, 8, 231,
				149, 158, 216, 194, 200, 62, 15, 53, 200, 188, 207, 0, 79, 211, 88, 194, 211, 54,
				1, 206, 53, 72, 118, 155, 184, 233, 166, 245, 224, 16, 254, 209, 235, 153, 85, 53,
				145, 33, 186, 218, 118, 144, 35, 189, 241, 63, 229, 52, 237, 231, 39, 176, 202, 93,
				247, 85, 131, 16, 193, 247, 180, 33, 138, 255, 102, 190, 213, 129, 174, 182, 167,
				3, 126, 184, 221, 99, 114, 238, 219, 157, 125, 230, 179, 160, 89, 202, 230, 16, 91,
				199, 57, 158, 225, 142, 125, 12, 211, 164, 78, 9, 4, 155, 106, 157, 41, 233, 188,
				237, 205, 184, 53, 0, 190, 24, 215, 42, 44, 184, 120, 58, 196, 198, 190, 114, 50,
				98, 240, 15, 213, 77, 163, 24, 3, 212, 125, 93, 175, 169, 249, 24, 27, 191, 113,
				89, 59, 169, 40, 87, 250, 144, 159, 118, 171, 232, 92, 217, 5, 179, 152, 249, 247,
				71, 239, 26, 180, 82, 177, 226, 132, 185, 3, 33, 162, 120, 98, 87, 109, 57, 100,
				202, 162, 57, 230, 44, 31, 63, 213, 30, 222, 241, 78, 162, 118, 120, 70, 196, 128,
				72, 223, 110, 5, 17, 151, 97, 214, 43, 57, 157, 1, 59, 87, 96, 17, 159, 174, 144,
				217, 159, 87, 36, 113, 41, 155, 186, 252, 162, 46, 22, 80, 133, 3, 113, 248, 11,
				118, 144, 155, 188, 77, 166, 40, 119, 107, 15, 233, 47, 47, 101, 77, 167, 141, 235,
				148, 34, 218, 164, 168, 71, 20, 239, 71, 24, 12, 109, 146, 232, 243, 65, 31, 72,
				186, 131, 190, 43, 227, 157, 41, 49, 126, 136, 51, 41, 50, 213, 37, 186, 223, 87,
				248, 34, 43, 132, 34, 0, 143, 75, 79, 43, 74, 183, 26, 2, 168, 53, 203, 208, 159,
				69, 107, 124, 33, 68, 113, 206, 127, 216, 158, 15, 52, 206, 1, 101, 109, 199, 13,
				131, 122, 29, 131, 133, 125, 219, 70, 69, 144, 133, 68, 233, 67, 203, 132, 160,
				143, 101, 84, 110, 15, 175, 111, 124, 24, 185, 222, 154, 238, 77, 241, 105, 8, 224,
				230, 43, 178, 49, 95, 137, 33, 227, 118, 207, 239, 56, 21, 51, 220, 22, 48, 162,
				22, 118, 229, 215, 248, 112, 198, 126, 180, 27, 161, 237, 56, 2, 220, 129, 126, 11,
				104, 8, 133, 190, 162, 204, 3, 63, 249, 173, 210, 152, 252, 143, 157, 79, 228, 232,
				230, 72, 164, 131, 183, 151, 230, 219, 186, 21, 34, 154, 219, 215, 231, 179, 47,
				217, 44, 115, 203, 157, 35, 195, 113, 235, 194, 102, 96, 205, 24, 221, 213, 147,
				120, 178, 221, 153, 146, 44, 172, 131, 77, 21, 61, 15, 5, 6, 205, 164, 203, 76,
				228, 29, 126, 136, 88, 230, 210, 62, 164, 103, 125, 55, 231, 129, 89, 61, 222, 50,
				71, 71, 75, 230, 70, 80, 85, 193, 136, 183, 222, 146, 46, 235, 0, 222, 118, 32, 70,
				85, 39, 92, 233, 211, 169, 159, 207, 145, 13, 206, 125, 3, 45, 51, 64, 167, 179,
				133, 83, 57, 190, 51, 239, 211, 74, 116, 75, 71, 248, 249, 184, 13, 31, 129, 107,
				104, 179, 76, 194, 186, 4, 13, 122, 167, 254, 126, 153, 50, 8, 1, 200, 203, 213,
				230, 217, 97, 105, 50, 208, 126, 180, 113, 81, 152, 238, 123, 157, 232, 19, 164,
				159, 164, 89, 75, 33, 70, 140, 204, 158, 236, 10, 226, 102, 14, 88, 134, 82, 131,
				36, 195, 127, 158, 81, 252, 223, 165, 11, 52, 105, 245, 245, 228, 235, 168, 175,
				52, 175, 76, 157, 120, 208, 99, 135, 210, 81, 114, 230, 181,
			],
		},
	};
	gen.with_reward(output, kernel)
}

/// Placeholder for mainnet genesis block, will definitely change before
/// release so no use trying to pre-mine it.
pub fn genesis_main() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader {

		height: 0,
timestamp: Utc.ymd(2019, 1, 29).and_hms(22, 52, 53),
prev_root: Hash::from_hex("00000000000000000014214ce74bf7a23ffe9d5edfbd77ce13347ddaded05a28").unwrap(),
output_root: Hash::from_hex("a6a9dc63542857088184ac7ec78e7f0614a966cd2405ee8f2143c136e217f17b").unwrap(),
range_proof_root: Hash::from_hex("17fdffe23c35416c7b74c5de675d917e1876d63b23fbf21c58d5ff39c4638589").unwrap(),
kernel_root: Hash::from_hex("2491086938ef970dff77f09487eff219d009b5c9bc6d56fb344877f763f2d060").unwrap(),
total_kernel_offset: BlindingFactor::from_hex("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(34)),
			secondary_scaling: 1856,
nonce: 12,
			proof: Proof {
nonces: vec![5766724, 12589870, 88642349, 94708577, 99298387, 129556331, 156412295, 208367253, 229776213, 258860196, 281505588, 282517059, 290508401, 301708851, 302137093, 304992878, 312000396, 321714757, 324304487, 326767079, 330853242, 338766398, 338949860, 344734035, 345959485, 355649919, 366632915, 372093800, 397255419, 400497048, 405510502, 422281496, 427081381, 433928170, 452357382, 454586907, 456857492, 496287482, 501556545, 504116908, 507403747, 517942103],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
excess: Commitment::from_vec(util::from_hex("084af37d9fa253e1c3bfa00576016bb9ab038224dcc2d1cdead40274ba40ff98e4".to_string()).unwrap()),
excess_sig: Signature::from_raw_data(&[217, 0, 224, 245, 25, 106, 91, 251, 96, 209, 15, 246, 201, 132, 182, 120, 251, 146, 204, 228, 4, 116, 185, 81, 160, 177, 135, 83, 110, 240, 216, 12, 247, 193, 160, 30, 41, 168, 234, 123, 232, 10, 7, 163, 237, 37, 186, 245, 116, 172, 206, 55, 190, 241, 4, 121, 60, 51, 255, 190, 153, 254, 238, 167]).unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
commit: Commitment::from_vec(util::from_hex("084d122e07f693e2f29b64e56d8253bc8b005b1cce16c7d1374501a728ec88fb62".to_string()).unwrap()),



		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [33, 237, 157, 164, 16, 248, 49, 226, 116, 214, 21, 193, 20, 22, 26, 236, 221, 184, 243, 96, 222, 42, 111, 74, 198, 215, 71, 27, 202, 133, 18, 19, 70, 109, 115, 123, 5, 22, 96, 71, 214, 68, 55, 178, 244, 79, 145, 51, 27, 210, 63, 191, 31, 64, 160, 20, 164, 58, 207, 139, 220, 184, 156, 104, 10, 169, 153, 57, 240, 40, 10, 129, 179, 139, 169, 10, 209, 9, 3, 141, 82, 203, 65, 159, 124, 141, 133, 147, 105, 192, 247, 190, 181, 36, 140, 10, 6, 96, 177, 209, 44, 123, 202, 73, 141, 101, 222, 125, 211, 229, 146, 166, 229, 227, 26, 58, 133, 160, 99, 252, 15, 162, 77, 149, 35, 86, 16, 197, 220, 43, 233, 124, 87, 39, 10, 219, 236, 182, 97, 24, 240, 148, 89, 102, 58, 176, 10, 163, 99, 203, 151, 90, 34, 49, 178, 90, 12, 170, 172, 25, 227, 118, 5, 8, 82, 184, 141, 190, 63, 11, 229, 203, 192, 171, 150, 72, 140, 83, 173, 54, 99, 238, 189, 210, 238, 25, 131, 7, 78, 199, 82, 68, 149, 147, 165, 138, 45, 43, 25, 129, 115, 19, 123, 207, 122, 23, 0, 193, 221, 214, 157, 183, 2, 102, 197, 129, 106, 213, 153, 163, 46, 2, 153, 6, 80, 191, 202, 125, 38, 30, 10, 180, 115, 239, 114, 253, 197, 56, 123, 93, 102, 38, 199, 20, 251, 179, 20, 95, 250, 222, 81, 56, 161, 142, 80, 164, 192, 183, 155, 24, 196, 162, 4, 175, 209, 33, 95, 226, 5, 45, 147, 66, 64, 230, 240, 65, 63, 134, 71, 59, 136, 35, 132, 235, 56, 10, 202, 62, 254, 0, 88, 131, 120, 109, 5, 133, 238, 133, 143, 213, 176, 158, 142, 2, 78, 110, 79, 61, 171, 123, 99, 172, 61, 227, 58, 200, 187, 194, 3, 108, 230, 131, 98, 58, 118, 120, 237, 94, 220, 151, 109, 89, 203, 7, 203, 88, 165, 44, 216, 226, 247, 144, 125, 139, 182, 104, 154, 74, 37, 191, 188, 202, 84, 219, 3, 101, 82, 79, 41, 87, 248, 253, 254, 250, 214, 155, 102, 97, 36, 238, 43, 244, 15, 231, 189, 150, 43, 234, 79, 136, 41, 155, 239, 12, 4, 84, 100, 140, 191, 50, 124, 95, 182, 150, 203, 70, 138, 119, 49, 162, 115, 142, 127, 173, 132, 32, 165, 121, 140, 54, 52, 106, 160, 197, 182, 131, 103, 84, 132, 170, 128, 131, 208, 152, 252, 208, 225, 154, 86, 33, 164, 210, 184, 252, 100, 255, 171, 213, 9, 59, 46, 41, 107, 186, 69, 101, 176, 144, 238, 161, 85, 40, 8, 41, 67, 109, 119, 37, 194, 135, 222, 110, 229, 243, 37, 233, 244, 227, 31, 223, 83, 230, 20, 77, 236, 29, 29, 148, 146, 158, 186, 185, 236, 165, 61, 96, 3, 198, 210, 7, 187, 127, 127, 231, 224, 116, 83, 203, 68, 230, 194, 39, 167, 200, 27, 144, 11, 76, 81, 197, 71, 219, 95, 86, 202, 6, 114, 67, 18, 136, 202, 25, 1, 152, 78, 246, 18, 185, 175, 156, 239, 56, 119, 187, 182, 83, 104, 211, 194, 98, 75, 67, 253, 5, 32, 62, 192, 255, 121, 53, 24, 199, 111, 238, 96, 35, 100, 181, 137, 93, 33, 195, 136, 189, 84, 173, 159, 160, 20, 37, 207, 33, 122, 244, 62, 142, 239, 89, 179, 182, 30, 116, 207, 105, 124, 135, 108, 79, 19, 98, 248, 38, 150, 178, 196, 236, 166, 101, 26, 138, 24, 180, 168, 93, 236, 128, 23, 195, 100, 44, 13, 239, 87, 103, 215, 167, 169, 243, 191, 48, 247, 48, 48, 243, 80, 142, 14, 151, 117, 125, 60, 45, 197, 109, 44, 105, 31, 30, 218, 53, 106, 243, 85, 134, 134, 68, 12, 235, 8, 231, 242, 105, 138, 80, 123, 135, 143, 49, 65, 197, 81, 51, 98, 146, 0, 159, 202, 5, 61, 108, 158, 138, 33, 191, 203],
		},
	};
	gen.with_reward(output, kernel)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::core::hash::Hashed;
	use crate::ser;

	#[test]
	fn floonet_genesis_hash() {
		let gen_hash = genesis_floo().hash();
		println!("floonet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_floo()).unwrap();
		println!("floonet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"edc758c1370d43e1d733f70f58cf187c3be8242830429b1676b89fd91ccf2dab"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"91c638fc019a54e6652bd6bb3d9c5e0c17e889cef34a5c28528e7eb61a884dc4"
		);
	}

	#[test]
	fn mainnet_genesis_hash() {
		let gen_hash = genesis_main().hash();
		println!("mainnet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_main()).unwrap();
		println!("floonet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"40adad0aec27797b48840aa9e00472015c21baea118ce7a2ff1a82c0f8f5bf82"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"6be6f34b657b785e558e85cc3b8bdb5bcbe8c10e7e58524c8027da7727e189ef"
		);
	}
}
