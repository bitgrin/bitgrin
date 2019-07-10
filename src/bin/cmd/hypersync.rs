// #[macro_use] extern crate guard;
use std::io::{Read};
use std::path::{Path};
use std::fs::{self, File};

use reqwest::{header, Client, Url};

use bitgrin_config as config;
use bitgrin_core::global;
use bitgrin_servers as servers;
use bitgrin_util::zip as bitgrin_zip;

enum HyperSyncState {
	NeedsExtract,
	NeedsDownload,
	NotNeeded,
}

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

fn expected_file(path: &Path) -> bool {
	true
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
	let uri = "https://d1joz5daoz8ntk.cloudfront.net/bg_chain_data07092019.zip";

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

pub fn try_hypersync() {
    println!("Hypersync...");
    // Retrieve common paths used for hyper-sync stages
	let server_config =	get_server_config();
	let db_root = Path::new(&server_config.db_root);
	guard!(let Some(db_parent_path) = db_root.parent()
		   else { println!("No db_root."); return; });
	guard!(let zip_path_root = Path::new(&db_parent_path)
	       else { println!("No db_parent_path"); return; });
	guard!(let zip_path = zip_path_root.join("bg_chain_data.zip")
	       else { println!("No db_parent_path"); return; });

    match should_perform_hyper_sync(db_root, &zip_path) {
        HyperSyncState::NeedsDownload => { start_hyper_sync(); do_extract(&zip_path, db_root); },
        HyperSyncState::NeedsExtract => { do_extract(&zip_path, db_root); },
        HyperSyncState::NotNeeded => { println!("Skipping hyper-sync."); }
    };
}