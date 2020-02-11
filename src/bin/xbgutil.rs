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
	let url = format!("http://mainseed.bitgrin.io:8513/v1/blocks/{}", block_num);
	match try_get_seed_hash(&url) {
		Ok(res) => Ok(res),
		Err(_) => {
			println!("Could not get main seed hash from mainseed, trying mainseed2...");
			let url2 = format!("http://mainseed2.bitgrin.io:8513/v1/blocks/{}", block_num);
			try_get_seed_hash(&url2)
		}
	}
}

/// Retrieve chain data for hyper-sync
use bitgrin_config as config;
use bitgrin_servers as servers;
use bitgrin_core::global;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
#[macro_use] extern crate guard;

use std::io::{Read};
use reqwest::{header, Client, Url};

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

use std::io;
use bitgrin_util::zip as bitgrin_zip;

fn expected_file(path: &Path) -> bool {
	true
	/*use lazy_static::lazy_static;
	use regex::Regex;
	let s_path = path.to_str().unwrap_or_else(|| "");
	lazy_static! {
		static ref RE: Regex = Regex::new(
			format!(
				r#"^({}|{}|{})((/|\\)pmmr_(hash|data|leaf|prun)\.bin(\.\w*)?)?$"#,
				OUTPUT_SUBDIR, KERNEL_SUBDIR, RANGE_PROOF_SUBDIR
			)
			.as_str()
		)
		.expect("invalid txhashset regular expression");
	}
	RE.is_match(&s_path)*/
}

fn do_extract(zip_path: &Path, target_dir: &Path) {
	guard!(let Ok(zip_file) = File::open(zip_path)
		   else {
			   println!("Could not open {:?} for extraction.", zip_path);
			   return;
		   });
	println!("Extrating chain data {:?} to {:?}", zip_file, target_dir);
	match bitgrin_zip::decompress(zip_file, target_dir, expected_file) {
		Ok(x) => { println!("OK! {}", x); },
		Err(e) => { println!("ERR: {:?}", e); },
	};
}

fn start_hyper_sync() {
	println!("Starting hyper-sync...");
	let uri = "https://d1joz5daoz8ntk.cloudfront.net/bg_chain_data02112020.zip";

	guard!(let Ok(url) = Url::parse(uri)
		   else { println!("Cannot parse URL"); return; });
	println!("Parsed URL");
    let client = Client::new();

    let total_size = {
        let resp_try = client.head(url.as_str()).send();
		match resp_try {
			Ok(resp) => {
				if resp.status().is_success() {
					resp.headers()
						.get(header::CONTENT_LENGTH)
						.and_then(|ct_len| ct_len.to_str().ok())
						.and_then(|ct_len| ct_len.parse().ok())
						.unwrap_or(0)
				} else {
					println!("Couldn't download URL: {}. Error: {:?}", url, resp.status()); -1
				}
			},
			Err(_) => { println!("Can't get size."); -1 }
		}
    };
	println!("Total size: {}", total_size);

    let mut request = client.get(url.as_str());

    let filename = Path::new(
         url
            .path_segments()
            .and_then(|segments| segments.last())
            .unwrap_or("tmp.bin"),
    );




	let server_config =	get_server_config();
	let db_root = Path::new(&server_config.db_root);
	
	guard!(let Some(db_parent_path) = db_root.parent()
		   else { println!("No db_root."); return; });
	guard!(let zip_path_root = Path::new(&db_parent_path)
	       else { println!("No db_parent_path"); return; });
	guard!(let zip_path = zip_path_root.join("bg_chain_data.zip")
	       else { println!("No db_parent_path"); return; });

    if zip_path.exists() {
		println!("file_exists");
		guard!(let Ok(zip_metadata) = zip_path.metadata()
			   else { println!("Couldnt get zip metadata."); return; });
        let size = zip_metadata.len() - 1;
        request = request.header(header::RANGE, format!("bytes={}-", size));
        //pb.inc(size);
		// println!("inc size {}", size);
    }

	guard!(let Ok(send_request) = request.send()
	       else { println!("send() error"); return; });

    let mut source = DownloadProgress {
        inner: send_request,
		bytes_downloaded: 0,
		out_per: 0.0,
		total_size: total_size as u64,
    };

	guard!(let Ok(mut dest) = fs::OpenOptions::new().create(true).append(true).open(&zip_path)
		   else { println!("err opening options"); return; });

    std::io::copy(&mut source, &mut dest);
	
    println!(
        "Download of '{}' has been completed.",
        zip_path.to_str().unwrap()
    );
}

/// Zips chain data for sharing to CDN
fn zip_chain_data(zip_path: &Path, target_dir: &Path) {
	// Delete peer folder
	
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

enum HyperSyncState {
	NeedsExtract,
	NeedsDownload,
	NotNeeded,
}

fn should_perform_hyper_sync(db_root: &Path, zip_path: &Path) -> HyperSyncState {
	// Check if bg_chain_data folder exists
	let chain_data_path = File::open(db_root);
	if let Ok(_) = chain_data_path {
		println!("Chain data folder exists, bailing hyper-sync.");
		return HyperSyncState::NotNeeded;
	}
	else {
		// if zip exist, skip to extraction
		let zip_file = File::open(zip_path.clone());
		if let Ok(_) = zip_file {
			println!("Skipping zip download as it already exists...");
			return HyperSyncState::NeedsExtract;
		}
		else {
			println!("No chain folder found, initiate hyper-sync");
			return HyperSyncState::NeedsDownload;
		}
	}
}

use std::env;

pub fn print_usage() {
	println!("      Usage: ");
	println!("      ./xbgutil CMD");
	println!("                   ");
	println!("      CMD Can be one of:");
	println!("cp / checkpoint:");
	println!("  Checkpoint the current blockchain state and save as a zip file");
	println!("hs / hypersync");
	println!("  Test hyper-sync functionality");
}

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
			"hypersync" | "hs" => {
				match should_perform_hyper_sync(db_root, &zip_path) {
					HyperSyncState::NeedsDownload => { start_hyper_sync(); do_extract(&zip_path, db_root); },
					HyperSyncState::NeedsExtract => { do_extract(&zip_path, db_root); },
					HyperSyncState::NotNeeded => { println!("Skipping hyper-sync."); }
				};
			}
			_ => { println!("Unrecognized command."); print_usage(); }
		}
	}
	else {
		print_usage();
	}

	/*
	// Seed bridge tests
	match seed_bridge() {
		Ok(x) => { println!("OK!: {}", x) },
		Err(e) => { println!("Err: {:?}", e)},
	}*/
}
