extern crate hyper;
extern crate tokio_timer;

use api::{BlockPrintable, Error, ErrorKind};
use bitgrin_api as api;
use bitgrin_core;
use bitgrin_core::core::hash::Hash;

use reqwest;
use std::time::Duration;

// fn get_hash() -> Result<Hash, Error> {
//     let block_num = 190027;
//     let url = format!("http://mainseed.bitgrin.io:8513/v1/blocks/{}", block_num);
//     let res = api::client::get_with_timeout::<api::BlockPrintable>(&url, None, 4);
//     println!("Attempting to get block hashes from mainseed");
//     match res {
//         Ok(block) => { return Hash::from_hex(&block.header.hash).map_err(|_| ErrorKind::NotFound.into()); },
//         Err(_) => {
//             // Try again
//             println!("Error getting block hash from mainseed, trying mainseed2");
//             let url2 = format!("http://mainseed2.bitgrin.io:8513/v1/blocks/{}", block_num);
//             let res2 = api::client::get::<api::BlockPrintable>(&url2, None);
//             match res2 {
//                 Ok(block2) => { return Hash::from_hex(&block2.header.hash).map_err(|_| ErrorKind::NotFound.into()); }
//                 Err(_) => { println!("err");return Err(ErrorKind::NotFound)? }
//             }
//         }
//     }
// }

fn try_get_seed_hash(url: &str) -> Result<Hash, Error> {
	let client = reqwest::Client::builder()
		.timeout(Duration::from_secs(4))
		.build();
	match client {
		Ok(c) => {
			let r = c.get(url).send();
			match r {
				Ok(mut res) => {
					let parsed_result: Result<BlockPrintable, reqwest::Error> = res.json();
					match parsed_result {
						Ok(parsed) => {
							println!("Parsed {}", &parsed.header.hash);
							Hash::from_hex(&parsed.header.hash)
								.map_err(|_| ErrorKind::NotFound.into())
						}
						Err(_) => {
							println!("Error parsing json");
							Err(ErrorKind::NotFound.into())
						}
					}
				}
				Err(e) => {
					println!("Error making web request to seed hash: {}", e);
					Err(ErrorKind::NotFound.into())
				}
			}
		}
		Err(e) => {
			println!("Error creating reqwest client: {}", e);
			Err(ErrorKind::NotFound.into())
		}
	}
}

pub fn seed_bridge() -> Result<Hash, Error> {
	let block_num = 190027;
	let url = format!("http://mainseed.bitgrin.io:8513/v1/blocks/{}", block_num);
	match try_get_seed_hash(&url) {
		Ok(res) => Ok(res),
		Err(e) => {
			println!("Could not get main seed hash from mainseed, trying mainseed2...");
			let url2 = format!("http://mainseed2.bitgrin.io:8513/v1/blocks/{}", block_num);
			try_get_seed_hash(&url2)
		}
	}
}

pub fn main() {}
