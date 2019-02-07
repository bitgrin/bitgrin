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

use crate::consensus::INITIAL_DIFFICULTY;
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
		timestamp: Utc.ymd(2019, 2, 7).and_hms(23, 0, 2),
		prev_root: Hash::from_hex(
			"000000000000000000071ea136532be6c7ff7d776d8a405648880b0591365ac5",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"c46815961364345c5a8dc51877c9972cde48c521f10a62e3d31b12b86af706fa",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"d7f8bf4ebfa56aa632378e38b7769a9387dba488727060970b69677b99690693",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"e6a452481a4f413cb63f9ce9ce7009941d206611e6bc7cb25084179a6049214b",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(INITIAL_DIFFICULTY),
			secondary_scaling: 1856,
			nonce: 12,
			proof: Proof {
				nonces: vec![
					5153291, 16383296, 40755350, 59409469, 63358041, 70894323, 72170551, 102196372,
					102916847, 108731761, 123023570, 130286589, 134710543, 144128563, 157821627,
					183628949, 194160882, 195498479, 210806627, 235299048, 241963375, 242734730,
					249846173, 252988986, 293836840, 326229091, 345493982, 347510258, 378524546,
					389539966, 402864118, 406401225, 426149625, 431266347, 432063002, 437585765,
					457280930, 492066015, 498424457, 507365347, 528024867, 532761433,
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
				"08266e75f5438685a5e928e72c356022ff88992f5cade787ebecf210fdf6666715".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[
			117, 116, 0, 192, 86, 155, 74, 233, 78, 146, 87, 113, 25, 133, 200, 185, 61, 225, 131,
			30, 53, 135, 75, 126, 44, 166, 11, 208, 87, 12, 77, 151, 111, 103, 106, 65, 241, 115,
			217, 216, 120, 195, 226, 200, 15, 43, 59, 44, 47, 225, 104, 71, 17, 138, 27, 214, 137,
			199, 16, 117, 86, 196, 161, 144,
		])
		.unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(
				"0985433892f86080b67b4c6195f5c90be0c119a437de8485602752af6fb2e0ffc8".to_string(),
			)
			.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [
				87, 27, 118, 123, 199, 10, 211, 23, 220, 186, 24, 239, 4, 212, 146, 212, 211, 28,
				65, 74, 250, 231, 22, 141, 250, 202, 15, 101, 62, 176, 233, 177, 75, 215, 160, 80,
				196, 47, 188, 240, 109, 214, 231, 140, 6, 157, 16, 0, 216, 197, 100, 106, 97, 162,
				188, 76, 199, 68, 141, 46, 239, 167, 121, 114, 1, 31, 18, 115, 88, 20, 121, 85, 30,
				46, 236, 115, 222, 85, 214, 96, 24, 3, 106, 136, 214, 141, 89, 128, 75, 227, 105,
				182, 124, 180, 131, 37, 13, 191, 91, 171, 96, 111, 200, 102, 216, 64, 158, 209,
				240, 173, 59, 142, 119, 60, 49, 130, 133, 95, 147, 204, 116, 251, 174, 93, 29, 161,
				133, 107, 146, 50, 139, 194, 124, 136, 133, 210, 215, 109, 156, 193, 109, 176, 118,
				32, 50, 215, 208, 182, 68, 129, 110, 75, 227, 67, 224, 116, 203, 70, 180, 187, 148,
				78, 146, 193, 114, 100, 30, 156, 113, 153, 81, 57, 67, 94, 72, 68, 160, 98, 143,
				150, 60, 199, 135, 70, 127, 196, 184, 43, 99, 103, 27, 92, 23, 199, 40, 104, 38,
				250, 2, 112, 30, 204, 52, 66, 79, 3, 155, 232, 95, 251, 114, 231, 80, 10, 23, 194,
				38, 11, 245, 201, 15, 207, 233, 3, 236, 62, 76, 198, 80, 141, 71, 48, 186, 237,
				193, 12, 214, 2, 195, 146, 26, 16, 145, 170, 7, 104, 158, 219, 87, 246, 228, 210,
				153, 87, 15, 219, 187, 215, 174, 17, 19, 221, 223, 233, 177, 58, 185, 210, 148,
				121, 41, 192, 89, 252, 236, 233, 49, 161, 233, 126, 62, 83, 30, 49, 13, 58, 211,
				152, 190, 233, 115, 223, 18, 83, 100, 99, 251, 133, 95, 231, 161, 8, 253, 67, 231,
				161, 88, 104, 11, 178, 166, 221, 148, 7, 128, 150, 145, 142, 152, 58, 127, 63, 211,
				207, 247, 221, 141, 67, 71, 95, 199, 5, 18, 238, 144, 93, 95, 161, 149, 73, 11, 99,
				243, 194, 92, 154, 61, 241, 250, 235, 231, 52, 202, 208, 3, 126, 19, 13, 110, 70,
				237, 139, 177, 122, 165, 154, 91, 126, 106, 53, 168, 161, 18, 188, 228, 34, 150,
				108, 87, 176, 142, 38, 174, 102, 159, 82, 19, 16, 94, 227, 250, 231, 120, 235, 26,
				24, 64, 105, 88, 250, 59, 36, 240, 236, 187, 193, 61, 84, 124, 27, 21, 0, 92, 40,
				139, 33, 112, 146, 159, 145, 187, 201, 130, 41, 105, 218, 144, 110, 159, 208, 164,
				156, 8, 205, 64, 187, 254, 219, 97, 34, 44, 92, 123, 191, 199, 253, 213, 244, 210,
				121, 87, 57, 189, 69, 164, 28, 244, 96, 175, 162, 136, 206, 96, 38, 43, 152, 52,
				235, 174, 147, 73, 17, 144, 156, 220, 193, 48, 158, 126, 148, 31, 223, 148, 224,
				84, 83, 213, 98, 205, 242, 155, 6, 88, 39, 168, 20, 4, 162, 28, 12, 73, 145, 230,
				231, 85, 113, 107, 115, 12, 41, 4, 236, 206, 59, 225, 68, 168, 142, 18, 179, 49,
				21, 34, 80, 170, 57, 180, 215, 134, 153, 33, 240, 88, 161, 36, 84, 223, 201, 153,
				154, 183, 24, 151, 233, 246, 225, 205, 240, 187, 28, 243, 227, 200, 187, 4, 101,
				130, 242, 128, 0, 191, 236, 26, 79, 43, 14, 83, 114, 41, 244, 151, 170, 137, 206,
				112, 156, 240, 167, 194, 156, 156, 172, 224, 81, 183, 27, 82, 235, 141, 222, 238,
				5, 55, 9, 210, 31, 188, 79, 235, 40, 84, 133, 142, 39, 20, 89, 131, 245, 232, 115,
				36, 81, 142, 231, 123, 226, 67, 121, 163, 150, 23, 38, 98, 72, 76, 170, 44, 224,
				201, 107, 172, 127, 116, 35, 11, 54, 81, 186, 208, 205, 128, 106, 250, 198, 114,
				124, 163, 136, 24, 175, 134, 203, 70, 59, 180, 52, 217, 118, 72, 89, 241, 35, 200,
				53, 176, 132, 25, 191, 78, 29, 3, 255, 60, 236, 109,
			],
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
