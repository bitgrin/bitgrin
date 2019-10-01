// Copyright 2018 The BitGrin Developers
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

//! Base types that the block chain pipeline requires.

use crate::core::core::hash::{Hash, Hashed, ZERO_HASH};
//use crate::core::core::{Block, BlockHeader};
use crate::core::core::{Block};
use crate::core::{core, ser};
use crate::core::core::merkle_proof::MerkleProof;
use crate::core::pow::Difficulty;
use crate::util;
use crate::util::secp::pedersen;
use std::sync::Arc;
use serde;
use serde::de::MapAccess;
use serde::ser::SerializeStruct;
use crate::{Error, Chain};
use std::fmt;

macro_rules! no_dup {
	($field:ident) => {
		if $field.is_some() {
			return Err(serde::de::Error::duplicate_field("$field"));
			}
	};
}

bitflags! {
/// Options for block validation
	pub struct Options: u32 {
		/// No flags
		const NONE = 0b00000000;
		/// Runs without checking the Proof of Work, mostly to make testing easier.
		const SKIP_POW = 0b00000001;
		/// Adds block while in syncing mode.
		const SYNC = 0b00000010;
		/// Block validation on a block we mined ourselves
		const MINE = 0b00000100;
	}
}

/// A helper to hold the roots of the txhashset in order to keep them
/// readable.
#[derive(Debug)]
pub struct TxHashSetRoots {
	/// Header root
	pub header_root: Hash,
	/// Output root
	pub output_root: Hash,
	/// Range Proof root
	pub rproof_root: Hash,
	/// Kernel root
	pub kernel_root: Hash,
}

/// The tip of a fork. A handle to the fork ancestry from its leaf in the
/// blockchain tree. References the max height and the latest and previous
/// blocks
/// for convenience and the total difficulty.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tip {
	/// Height of the tip (max height of the fork)
	pub height: u64,
	/// Last block pushed to the fork
	pub last_block_h: Hash,
	/// Previous block
	pub prev_block_h: Hash,
	/// Total difficulty accumulated on that fork
	pub total_difficulty: Difficulty,
}

impl Tip {
	/// Creates a new tip based on provided header.
	pub fn from_header(header: &core::BlockHeader) -> Tip {
		Tip {
			height: header.height,
			last_block_h: header.hash(),
			prev_block_h: header.prev_hash,
			total_difficulty: header.total_difficulty(),
		}
	}
}

impl Hashed for Tip {
	/// The hash of the underlying block.
	fn hash(&self) -> Hash {
		self.last_block_h
	}
}

impl Default for Tip {
	fn default() -> Self {
		Tip {
			height: 0,
			last_block_h: ZERO_HASH,
			prev_block_h: ZERO_HASH,
			total_difficulty: Difficulty::min(),
		}
	}
}

/// Serialization of a tip, required to save to datastore.
impl ser::Writeable for Tip {
	fn write<W: ser::Writer>(&self, writer: &mut W) -> Result<(), ser::Error> {
		writer.write_u64(self.height)?;
		writer.write_fixed_bytes(&self.last_block_h)?;
		writer.write_fixed_bytes(&self.prev_block_h)?;
		self.total_difficulty.write(writer)
	}
}

impl ser::Readable for Tip {
	fn read(reader: &mut dyn ser::Reader) -> Result<Tip, ser::Error> {
		let height = reader.read_u64()?;
		let last = Hash::read(reader)?;
		let prev = Hash::read(reader)?;
		let diff = Difficulty::read(reader)?;
		Ok(Tip {
			height: height,
			last_block_h: last,
			prev_block_h: prev,
			total_difficulty: diff,
		})
	}
}

/// Bridge between the chain pipeline and the rest of the system. Handles
/// downstream processing of valid blocks by the rest of the system, most
/// importantly the broadcasting of blocks to our peers.
pub trait ChainAdapter {
	/// The blockchain pipeline has accepted this block as valid and added
	/// it to our chain.
	fn block_accepted(&self, block: &Block, status: BlockStatus, opts: Options);
}

/// Inform the caller of the current status of a txhashset write operation,
/// as it can take quite a while to process. Each function is called in the
/// order defined below and can be used to provide some feedback to the
/// caller. Functions taking arguments can be called repeatedly to update
/// those values as the processing progresses.
pub trait TxHashsetWriteStatus {
	/// First setup of the txhashset
	fn on_setup(&self);
	/// Starting validation
	fn on_validation(&self, kernels: u64, kernel_total: u64, rproofs: u64, rproof_total: u64);
	/// Starting to save the txhashset and related data
	fn on_save(&self);
	/// Done writing a new txhashset
	fn on_done(&self);
}

/// Do-nothing implementation of TxHashsetWriteStatus
pub struct NoStatus;

impl TxHashsetWriteStatus for NoStatus {
	fn on_setup(&self) {}
	fn on_validation(&self, _ks: u64, _kts: u64, _rs: u64, _rt: u64) {}
	fn on_save(&self) {}
	fn on_done(&self) {}
}

/// Dummy adapter used as a placeholder for real implementations
pub struct NoopAdapter {}

impl ChainAdapter for NoopAdapter {
	fn block_accepted(&self, _b: &Block, _status: BlockStatus, _opts: Options) {}
}

/// Status of an accepted block.
#[derive(Debug, Clone, PartialEq)]
pub enum BlockStatus {
	/// Block is the "next" block, updating the chain head.
	Next,
	/// Block does not update the chain head and is a fork.
	Fork,
	/// Block updates the chain head via a (potentially disruptive) "reorg".
	/// Previous block was not our previous chain head.
	Reorg(u64),
	/// Chain hash unexpected
	ChainIntegrityFailure,
}




/// ****
/// Just enough info from api crate to recreate BlockPrintable
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OutputType {
	/// Coinbase output type
	Coinbase,
	/// Transaction output type
	Transaction,
}

/// As above, except formatted a bit better for human viewing
#[derive(Debug, Clone)]
pub struct OutputPrintable {
	/// The type of output Coinbase|Transaction
	pub output_type: OutputType,
	/// The homomorphic commitment representing the output's amount
	/// (as hex string)
	pub commit: pedersen::Commitment,
	/// Whether the output has been spent
	pub spent: bool,
	/// Rangeproof (as hex string)
	pub proof: Option<String>,
	/// Rangeproof hash (as hex string)
	pub proof_hash: String,
	/// Block height at which the output is found
	pub block_height: Option<u64>,
	/// Merkle Proof
	pub merkle_proof: Option<MerkleProof>,
	/// MMR Position
	pub mmr_index: u64,
}

/// Printable form of an output
impl OutputPrintable {
	/// Create printable from output
	pub fn from_output(
		output: &core::Output,
		chain: Arc<Chain>,
		block_header: Option<&core::BlockHeader>,
		include_proof: bool,
		include_merkle_proof: bool,
	) -> Result<OutputPrintable, Error> {
		let output_type = if output.is_coinbase() {
			OutputType::Coinbase
		} else {
			OutputType::Transaction
		};

		let out_id = core::OutputIdentifier::from_output(&output);
		let spent = chain.is_unspent(&out_id).is_err();
		let block_height = match spent {
			true => None,
			false => Some(chain.get_header_for_output(&out_id)?.height),
		};

		let proof = if include_proof {
			Some(util::to_hex(output.proof.proof.to_vec()))
		} else {
			None
		};

		// Get the Merkle proof for all unspent coinbase outputs (to verify maturity on
		// spend). We obtain the Merkle proof by rewinding the PMMR.
		// We require the rewind() to be stable even after the PMMR is pruned and
		// compacted so we can still recreate the necessary proof.
		let mut merkle_proof = None;
		if include_merkle_proof && output.is_coinbase() && !spent {
			if let Some(block_header) = block_header {
				merkle_proof = chain.get_merkle_proof(&out_id, &block_header).ok();
			}
		};

		let output_pos = chain.get_output_pos(&output.commit).unwrap_or(0);

		Ok(OutputPrintable {
			output_type,
			commit: output.commit,
			spent,
			proof,
			proof_hash: util::to_hex(output.proof.hash().to_vec()),
			block_height,
			merkle_proof,
			mmr_index: output_pos,
		})
	}

	/// Clones the commit
	pub fn commit(&self) -> Result<pedersen::Commitment, ser::Error> {
		Ok(self.commit.clone())
	}

	/// Validate RangeProof and return a clone
	pub fn range_proof(&self) -> Result<pedersen::RangeProof, ser::Error> {
		let proof_str = match self.proof.clone() {
			Some(p) => p,
			None => return Err(ser::Error::HexError(format!("output range_proof missing"))),
		};

		let p_vec = util::from_hex(proof_str)
			.map_err(|_| ser::Error::HexError(format!("invalud output range_proof")))?;
		let mut p_bytes = [0; util::secp::constants::MAX_PROOF_SIZE];
		for i in 0..p_bytes.len() {
			p_bytes[i] = p_vec[i];
		}
		Ok(pedersen::RangeProof {
			proof: p_bytes,
			plen: p_bytes.len(),
		})
	}
}

impl serde::ser::Serialize for OutputPrintable {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		let mut state = serializer.serialize_struct("OutputPrintable", 7)?;
		state.serialize_field("output_type", &self.output_type)?;
		state.serialize_field("commit", &util::to_hex(self.commit.0.to_vec()))?;
		state.serialize_field("spent", &self.spent)?;
		state.serialize_field("proof", &self.proof)?;
		state.serialize_field("proof_hash", &self.proof_hash)?;
		state.serialize_field("block_height", &self.block_height)?;

		let hex_merkle_proof = &self.merkle_proof.clone().map(|x| x.to_hex());
		state.serialize_field("merkle_proof", &hex_merkle_proof)?;
		state.serialize_field("mmr_index", &self.mmr_index)?;

		state.end()
	}
}

impl<'de> serde::de::Deserialize<'de> for OutputPrintable {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::de::Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(field_identifier, rename_all = "snake_case")]
		enum Field {
			OutputType,
			Commit,
			Spent,
			Proof,
			ProofHash,
			BlockHeight,
			MerkleProof,
			MmrIndex,
		}

		struct OutputPrintableVisitor;

		impl<'de> serde::de::Visitor<'de> for OutputPrintableVisitor {
			type Value = OutputPrintable;

			fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
				formatter.write_str("a print able Output")
			}

			fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
			where
				A: MapAccess<'de>,
			{
				let mut output_type = None;
				let mut commit = None;
				let mut spent = None;
				let mut proof = None;
				let mut proof_hash = None;
				let mut block_height = None;
				let mut merkle_proof = None;
				let mut mmr_index = None;

				while let Some(key) = map.next_key()? {
					match key {
						Field::OutputType => {
							no_dup!(output_type);
							output_type = Some(map.next_value()?)
						}
						Field::Commit => {
							no_dup!(commit);

							let val: String = map.next_value()?;
							let vec =
								util::from_hex(val.clone()).map_err(serde::de::Error::custom)?;
							commit = Some(pedersen::Commitment::from_vec(vec));
						}
						Field::Spent => {
							no_dup!(spent);
							spent = Some(map.next_value()?)
						}
						Field::Proof => {
							no_dup!(proof);
							proof = map.next_value()?
						}
						Field::ProofHash => {
							no_dup!(proof_hash);
							proof_hash = Some(map.next_value()?)
						}
						Field::BlockHeight => {
							no_dup!(block_height);
							block_height = Some(map.next_value()?)
						}
						Field::MerkleProof => {
							no_dup!(merkle_proof);
							if let Some(hex) = map.next_value::<Option<String>>()? {
								if let Ok(res) = MerkleProof::from_hex(&hex) {
									merkle_proof = Some(res);
								} else {
									merkle_proof = Some(MerkleProof::empty());
								}
							}
						}
						Field::MmrIndex => {
							no_dup!(mmr_index);
							mmr_index = Some(map.next_value()?)
						}
					}
				}

				if output_type.is_none()
					|| commit.is_none() || spent.is_none()
					|| proof_hash.is_none()
					|| mmr_index.is_none()
				{
					return Err(serde::de::Error::custom("invalid output"));
				}

				Ok(OutputPrintable {
					output_type: output_type.unwrap(),
					commit: commit.unwrap(),
					spent: spent.unwrap(),
					proof: proof,
					proof_hash: proof_hash.unwrap(),
					block_height: block_height,
					merkle_proof: merkle_proof,
					mmr_index: mmr_index.unwrap(),
				})
			}
		}

		const FIELDS: &'static [&'static str] = &[
			"output_type",
			"commit",
			"spent",
			"proof",
			"proof_hash",
			"mmr_index",
		];
		deserializer.deserialize_struct("OutputPrintable", FIELDS, OutputPrintableVisitor)
	}
}

/// Printable representation of a block
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TxKernelPrintable {
	/// Kernel features
	pub features: String,
	/// Fee for kernel
	pub fee: u64,
	/// Lock height of kernel
	pub lock_height: u64,
	/// Kernel excess
	pub excess: String,
	/// Kernel excess signature
	pub excess_sig: String,
}

/// Just the information required for wallet reconstruction
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockHeaderInfo {
	/// Hash
	pub hash: String,
	/// Height of this block since the genesis block (height 0)
	pub height: u64,
	/// Hash of the block previous to this in the chain.
	pub previous: String,
}

/// Printable representation of a block header
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockHeaderPrintable {
	/// Hash
	pub hash: String,
	/// Version of the block
	pub version: u16,
	/// Height of this block since the genesis block (height 0)
	pub height: u64,
	/// Hash of the block previous to this in the chain.
	pub previous: String,
	/// Root hash of the header MMR at the previous header.
	pub prev_root: String,
	/// rfc3339 timestamp at which the block was built.
	pub timestamp: String,
	/// Merklish root of all the commitments in the TxHashSet
	pub output_root: String,
	/// Merklish root of all range proofs in the TxHashSet
	pub range_proof_root: String,
	/// Merklish root of all transaction kernels in the TxHashSet
	pub kernel_root: String,
	/// Nonce increment used to mine this block.
	pub nonce: u64,
	/// Size of the cuckoo graph
	pub edge_bits: u8,
	/// Nonces of the cuckoo solution
	pub cuckoo_solution: Vec<u64>,
	/// Total accumulated difficulty since genesis block
	pub total_difficulty: u64,
	/// Variable difficulty scaling factor for secondary proof of work
	pub secondary_scaling: u32,
	/// Total kernel offset since genesis block
	pub total_kernel_offset: String,
}

/// Printable representation of a block
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockPrintable {
	/// The block header
	pub header: BlockHeaderPrintable,
	/// Input transactions
	pub inputs: Vec<String>,
	/// A printable version of the outputs
	pub outputs: Vec<OutputPrintable>,
	/// A printable version of the transaction kernels
	pub kernels: Vec<TxKernelPrintable>,
}