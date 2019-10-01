use api::{BlockPrintable, Error, ErrorKind};
use bitgrin_api as api;
use bitgrin_core;
use bitgrin_core::core::hash::Hash;

use reqwest;
use std::time::Duration;

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
	let block_num = 215742;
	let url = format!("http://mainseed.bitgrin.dev:8513/v1/blocks/{}", block_num);
	match try_get_seed_hash(&url) {
		Ok(res) => Ok(res),
		Err(_) => {
			println!("Could not get main seed hash from mainseed, trying mainseed2...");
			let url2 = format!("http://mainseed2.bitgrin.dev:8513/v1/blocks/{}", block_num);
			try_get_seed_hash(&url2)
		}
	}
}

/// Retrieve chain data for hyper-sync
use bitgrin_config as config;
use bitgrin_servers as servers;
use bitgrin_core::global;
use std::fs::{File};
use std::path::{Path};
#[macro_use] extern crate guard;

use std::io::{Read};
// use reqwest::{header, Client, Url};

struct DownloadProgress<R> {
    inner: R,
    bytes_downloaded: u64,
	total_size: u64,
	out_per: f64,
}

impl<R: Read> DownloadProgress<R> {
	// Increment downloaded bytes
	fn inc(&mut self, n: u64) {
		self.bytes_downloaded += n;
		let per: f64 = self.bytes_downloaded as f64 / self.total_size as f64 * 100.0;
		if per - self.out_per > 5.0 {
			println!("Hyper-sync downloading chain state {}%", per);
			self.out_per = per;
		}
	}
}

impl<R: Read> Read for DownloadProgress<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf).map(|n| {
            self.inc(n as u64);
            n
        })
    }
}

fn get_server_config() -> servers::common::types::ServerConfig {
	let chain_type = global::ChainTypes::Mainnet;
	let node_config = Some(
		config::initial_setup_server(&chain_type).unwrap_or_else(|e| {
			panic!("Error loading server configuration: {}", e);
		}),
	);
	node_config.unwrap().members.as_ref().unwrap().server.clone()
}

use bitgrin_util::zip as bitgrin_zip;

/// Zips chain data for sharing to CDN
fn zip_chain_data(zip_path: &Path, target_dir: &Path) {
	guard!(let Ok(zip_file) = File::create(zip_path)
		   else {
			   println!("Could not open {:?} for extraction.", zip_path);
			   return;
		   });
	println!("zip {:?} in to {:?}", target_dir, zip_file);
	match bitgrin_zip::compress(target_dir, &zip_file) {
		Ok(e) => { println!("ok zipped! {:?}", e); },
		Err(e) => { println!("err zipping: {:?}", e); },
	}
}

use std::env;

pub fn main() {
	// Retrieve common paths used for hyper-sync stages
	let server_config =	get_server_config();
	let db_root = Path::new(&server_config.db_root);
	guard!(let Some(db_parent_path) = db_root.parent()
		   else { println!("No db_root."); return; });
	guard!(let zip_path_root = Path::new(&db_parent_path)
	       else { println!("No db_parent_path"); return; });
	guard!(let zip_path = zip_path_root.join("bg_chain_data.zip")
	       else { println!("No db_parent_path"); return; });

	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		// Passing commands to bridge
		let cmd = &args[1];
		println!("cmd: {}", cmd);
		match cmd.as_str() {
			"checkpoint" | "cp" => {
				zip_chain_data(&zip_path, db_root);
			}
			_ => { println!("Unrecognized command."); }
		}
	}
	// else {
	// 	match should_perform_hyper_sync(db_root, &zip_path) {
	// 		HyperSyncState::NeedsDownload => { start_hyper_sync(); do_extract(&zip_path, db_root); },
	// 		HyperSyncState::NeedsExtract => { do_extract(&zip_path, db_root); },
	// 		HyperSyncState::NotNeeded => { println!("Skipping hyper-sync."); }
	// 	};
	// }

	/*
	// Seed bridge tests
	match seed_bridge() {
		Ok(x) => { println!("OK!: {}", x) },
		Err(e) => { println!("Err: {:?}", e)},
	}*/
}
