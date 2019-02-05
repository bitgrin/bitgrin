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
timestamp: Utc.ymd(2019, 2, 5).and_hms(22, 56, 25),
prev_root: Hash::from_hex("000000000000000000116cd270ab06f1863d6e13d93e67ea551256567df51bff").unwrap(),
output_root: Hash::from_hex("0f026be82f8431b4867d883784180ad61b7b706de07c10c3732cfca7736605f4").unwrap(),
range_proof_root: Hash::from_hex("e50d8de7b48e375e85c9daa9eed870f448ab3a7bdbbc37cc287fc1fa1cc7f6d5").unwrap(),
kernel_root: Hash::from_hex("6e58f4c7862fc1a548555e7a0c410718e8276e9248beb3d055dc1500bc509da2").unwrap(),
total_kernel_offset: BlindingFactor::from_hex("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(INITIAL_DIFFICULTY),
			secondary_scaling: 1856,
nonce: 56,
			proof: Proof {
nonces: vec![4845939, 26017997, 42254355, 43102217, 43367409, 72348136, 75426230, 92848581, 93779587, 115763247, 118518698, 120630240, 149092006, 150466122, 158532585, 165719469, 177892075, 193593558, 195116119, 207818307, 208105533, 220377473, 247147262, 254083724, 271644007, 286438993, 330894286, 343140347, 348987425, 354851106, 365847173, 373461754, 379314718, 380824943, 383650814, 389965997, 406154374, 425919973, 441207093, 470584298, 504397357, 507014560],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
excess: Commitment::from_vec(util::from_hex("090afd3986b47ef42dc18a863a89bc739bf9d9fb60dc92cf49121502418649d0db".to_string()).unwrap()),
excess_sig: Signature::from_raw_data(&[229, 75, 164, 246, 231, 79, 237, 87, 238, 51, 120, 210, 21, 142, 208, 231, 59, 91, 10, 201, 185, 231, 213, 134, 207, 121, 123, 173, 27, 166, 239, 174, 181, 188, 137, 162, 231, 105, 245, 28, 12, 232, 137, 133, 65, 11, 60, 137, 96, 178, 222, 14, 248, 191, 23, 84, 130, 234, 78, 163, 188, 62, 113, 182]).unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
commit: Commitment::from_vec(util::from_hex("0947b9a953e7bc5d2ee81ece805d02642466bcedf87e5236a7143645609eeca16a".to_string()).unwrap()),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
proof: [161, 199, 21, 156, 11, 234, 126, 134, 168, 26, 174, 45, 43, 161, 102, 23, 153, 56, 6, 173, 97, 149, 143, 158, 101, 59, 16, 183, 137, 38, 207, 248, 14, 107, 226, 191, 216, 148, 60, 222, 19, 61, 37, 231, 200, 243, 12, 152, 205, 77, 183, 235, 15, 61, 128, 135, 196, 18, 100, 160, 72, 29, 77, 26, 14, 175, 233, 247, 9, 5, 175, 89, 231, 120, 60, 227, 96, 252, 184, 20, 135, 5, 195, 110, 182, 51, 166, 93, 203, 153, 128, 209, 188, 202, 203, 208, 139, 83, 3, 77, 64, 76, 242, 201, 222, 164, 147, 85, 230, 237, 77, 71, 86, 170, 253, 209, 207, 137, 214, 17, 126, 61, 254, 97, 203, 5, 167, 87, 173, 159, 81, 85, 11, 170, 219, 133, 242, 93, 16, 17, 232, 164, 39, 177, 126, 40, 247, 80, 27, 248, 32, 125, 144, 45, 52, 168, 74, 158, 139, 103, 32, 58, 198, 44, 255, 50, 244, 182, 16, 190, 250, 190, 25, 132, 209, 205, 188, 54, 68, 29, 74, 196, 59, 204, 168, 191, 88, 238, 117, 154, 70, 231, 151, 15, 142, 203, 6, 239, 199, 61, 180, 56, 241, 82, 189, 139, 161, 113, 125, 68, 164, 56, 11, 19, 149, 249, 53, 104, 98, 113, 243, 207, 104, 52, 176, 133, 246, 107, 46, 143, 142, 72, 179, 32, 134, 130, 123, 63, 192, 199, 169, 221, 142, 181, 9, 60, 133, 166, 174, 249, 197, 112, 25, 152, 202, 196, 20, 81, 26, 3, 2, 240, 77, 40, 131, 165, 209, 147, 217, 0, 36, 33, 252, 176, 6, 125, 215, 188, 21, 213, 61, 91, 211, 150, 76, 65, 205, 183, 65, 135, 112, 194, 241, 6, 80, 42, 208, 10, 113, 46, 181, 131, 10, 136, 249, 89, 133, 116, 172, 6, 212, 176, 79, 51, 75, 72, 124, 218, 216, 121, 3, 13, 117, 149, 125, 218, 142, 205, 203, 73, 251, 115, 174, 100, 156, 203, 196, 207, 187, 81, 80, 73, 159, 190, 243, 253, 83, 28, 22, 18, 236, 120, 252, 197, 2, 8, 143, 1, 27, 122, 24, 213, 183, 119, 240, 61, 48, 124, 13, 64, 231, 229, 38, 193, 195, 92, 149, 86, 195, 140, 46, 140, 95, 23, 41, 86, 124, 210, 115, 177, 120, 117, 29, 229, 99, 31, 254, 48, 221, 192, 243, 93, 17, 170, 152, 241, 198, 227, 231, 180, 30, 192, 80, 206, 166, 194, 58, 167, 27, 40, 190, 53, 44, 40, 221, 166, 221, 192, 57, 192, 222, 81, 141, 26, 116, 138, 119, 237, 197, 81, 205, 32, 224, 209, 178, 169, 73, 86, 86, 246, 90, 24, 65, 57, 114, 163, 83, 86, 42, 54, 231, 163, 55, 129, 248, 204, 231, 82, 163, 228, 123, 28, 77, 13, 62, 79, 152, 9, 40, 94, 159, 32, 112, 88, 160, 252, 40, 137, 151, 128, 19, 32, 104, 226, 132, 169, 80, 79, 153, 85, 21, 116, 98, 165, 240, 107, 207, 142, 35, 57, 45, 205, 239, 200, 170, 210, 218, 118, 80, 23, 30, 167, 132, 128, 38, 125, 184, 139, 124, 77, 155, 74, 0, 48, 238, 21, 94, 77, 217, 253, 98, 3, 134, 212, 246, 2, 60, 254, 144, 100, 194, 192, 182, 57, 94, 187, 87, 136, 84, 230, 35, 112, 64, 202, 128, 16, 187, 250, 58, 120, 96, 71, 142, 165, 109, 84, 207, 240, 10, 41, 64, 53, 26, 207, 206, 185, 120, 76, 6, 159, 81, 253, 101, 98, 4, 97, 4, 92, 121, 113, 217, 194, 83, 227, 110, 60, 137, 222, 97, 251, 80, 196, 234, 233, 21, 115, 248, 24, 202, 243, 125, 118, 5, 195, 15, 209, 96, 79, 125, 66, 202, 201, 252, 224, 46, 64, 227, 147, 160, 229, 187, 163, 62, 36, 227, 94, 130, 105, 148, 225, 161, 12, 51, 85, 124, 16, 160, 31, 194, 61, 37, 70, 78, 64, 164, 132, 69, 111, 66, 43, 219, 191, 4, 251, 186],
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
