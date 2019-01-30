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
use crate::consensus::{INITIAL_DIFFICULTY};

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
timestamp: Utc.ymd(2019, 1, 30).and_hms(17, 53, 43),
prev_root: Hash::from_hex("000000000000000000153d98049cfd7e04420c8a3c03fcc46373131a381734b7").unwrap(),
output_root: Hash::from_hex("17634bb5cc89466d5c63fb2e647842294a0c2a1e7bec205db3ec7ac291d7ec08").unwrap(),
range_proof_root: Hash::from_hex("0aa5ef55e5d42233c0a3d07b580bfefa059d9851a7f5bc0885a4ed15b053ced9").unwrap(),
kernel_root: Hash::from_hex("914c9d186ba0e08a8e366a09082b69aa92d8f40cab973e66b82cb0a05a7e08ca").unwrap(),
total_kernel_offset: BlindingFactor::from_hex("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(34)),
			secondary_scaling: 1856,
nonce: 72,
			proof: Proof {
nonces: vec![377179, 4150703, 17900774, 23967872, 74022982, 98380742, 100430981, 104442608, 106827719, 108279565, 110565828, 115438034, 117100687, 159873000, 179252520, 212320081, 224546685, 234008340, 239200470, 250759546, 259920426, 269865729, 284188228, 289548916, 295788852, 301543174, 305295441, 369588746, 373536764, 420711518, 428640646, 442841582, 444991374, 454542034, 456089705, 457418486, 475808815, 487047273, 501547470, 513909602, 522575551, 527917004],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
excess: Commitment::from_vec(util::from_hex("08b99c48c2e277486e965670ceec587e016e63ad220e32dd2557d15ab386fb0d7f".to_string()).unwrap()),
excess_sig: Signature::from_raw_data(&[37, 255, 122, 80, 156, 65, 79, 9, 163, 107, 147, 35, 30, 33, 59, 202, 187, 68, 228, 24, 20, 59, 167, 0, 7, 161, 5, 71, 55, 164, 140, 10, 139, 122, 146, 248, 229, 181, 90, 176, 87, 65, 39, 142, 162, 192, 203, 105, 39, 73, 43, 14, 255, 25, 39, 66, 127, 113, 232, 52, 20, 88, 135, 104]).unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
commit: Commitment::from_vec(util::from_hex("08985b0abc9eb48e8655a94f5cbfeeeb2f8f113ffae61b2fc14e591bf1bb5766bf".to_string()).unwrap()),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
proof: [94, 152, 24, 228, 248, 168, 182, 97, 212, 184, 162, 149, 51, 134, 60, 57, 19, 15, 155, 131, 86, 199, 165, 160, 87, 9, 26, 51, 54, 233, 42, 142, 222, 36, 219, 255, 201, 114, 210, 231, 148, 7, 7, 141, 126, 105, 250, 203, 37, 248, 136, 128, 144, 38, 200, 18, 100, 235, 98, 181, 221, 133, 33, 133, 6, 56, 51, 24, 73, 85, 107, 67, 202, 192, 127, 87, 146, 153, 92, 252, 145, 39, 240, 65, 122, 162, 233, 43, 221, 169, 189, 242, 7, 29, 144, 14, 231, 53, 193, 169, 182, 85, 21, 61, 41, 166, 130, 250, 21, 202, 11, 184, 214, 143, 221, 115, 6, 143, 19, 163, 59, 14, 57, 220, 165, 146, 250, 126, 211, 85, 11, 203, 77, 71, 11, 27, 96, 220, 73, 103, 132, 167, 122, 139, 186, 193, 145, 177, 148, 243, 125, 90, 35, 171, 136, 188, 171, 30, 220, 92, 247, 73, 17, 167, 100, 142, 155, 122, 6, 225, 228, 122, 43, 100, 185, 141, 163, 254, 38, 187, 179, 48, 23, 201, 133, 227, 87, 229, 229, 134, 49, 27, 141, 97, 15, 169, 3, 64, 75, 104, 82, 227, 207, 152, 131, 105, 162, 185, 47, 161, 255, 252, 174, 218, 135, 115, 64, 65, 55, 156, 66, 135, 148, 129, 209, 186, 14, 97, 117, 167, 49, 240, 166, 236, 123, 124, 136, 212, 145, 174, 206, 117, 157, 93, 253, 252, 115, 2, 48, 190, 57, 220, 69, 154, 113, 40, 98, 67, 57, 61, 177, 66, 72, 1, 121, 240, 5, 83, 47, 243, 197, 9, 99, 72, 69, 27, 138, 102, 166, 13, 187, 105, 178, 57, 13, 210, 11, 126, 50, 4, 245, 14, 240, 253, 86, 24, 177, 210, 233, 208, 109, 130, 229, 34, 134, 32, 237, 14, 231, 225, 58, 102, 7, 105, 96, 187, 153, 249, 188, 20, 74, 245, 121, 243, 127, 191, 111, 27, 45, 152, 217, 8, 177, 107, 111, 29, 186, 26, 132, 83, 217, 35, 255, 147, 43, 125, 192, 152, 48, 76, 128, 156, 109, 196, 3, 124, 215, 186, 20, 9, 62, 110, 236, 57, 38, 26, 50, 107, 89, 34, 86, 62, 134, 72, 245, 105, 221, 36, 100, 239, 208, 181, 109, 160, 201, 166, 49, 122, 119, 241, 55, 182, 160, 93, 170, 204, 79, 106, 39, 217, 125, 114, 117, 173, 82, 108, 70, 220, 154, 157, 137, 129, 152, 193, 22, 215, 11, 184, 249, 122, 217, 219, 187, 83, 42, 220, 120, 237, 207, 112, 72, 6, 78, 236, 27, 46, 219, 143, 207, 5, 14, 164, 142, 205, 132, 87, 35, 211, 153, 100, 3, 56, 108, 197, 255, 76, 196, 77, 117, 233, 73, 223, 63, 75, 24, 235, 177, 115, 49, 15, 56, 204, 206, 2, 58, 196, 179, 212, 116, 129, 61, 89, 218, 132, 154, 45, 192, 111, 79, 120, 198, 42, 16, 56, 106, 243, 128, 132, 57, 12, 255, 226, 38, 88, 84, 90, 59, 4, 5, 125, 252, 43, 76, 190, 240, 233, 67, 226, 144, 140, 26, 98, 107, 230, 62, 154, 252, 199, 120, 92, 112, 7, 4, 69, 63, 57, 171, 152, 177, 124, 130, 125, 27, 141, 223, 19, 226, 7, 178, 78, 197, 235, 226, 18, 249, 35, 70, 36, 156, 49, 161, 13, 83, 38, 120, 181, 175, 89, 217, 221, 175, 224, 212, 123, 1, 14, 39, 89, 248, 244, 146, 67, 14, 160, 143, 118, 182, 246, 127, 110, 255, 218, 49, 239, 212, 198, 61, 162, 22, 90, 161, 122, 95, 4, 53, 20, 120, 54, 252, 17, 98, 133, 35, 17, 145, 194, 3, 11, 180, 237, 26, 13, 100, 109, 48, 52, 8, 254, 9, 202, 169, 11, 89, 125, 138, 75, 54, 47, 18, 170, 138, 194, 72, 185, 143, 157, 76, 170, 239, 59, 195, 15, 59, 134, 141, 205, 232, 169, 89, 246, 66, 84, 39, 231, 139, 131, 186, 35, 32, 134, 120, 218, 88, 189, 128],
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
