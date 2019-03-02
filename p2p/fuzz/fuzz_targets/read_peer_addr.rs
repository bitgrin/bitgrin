#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate bitgrin_core;
extern crate bitgrin_p2p;

use bitgrin_core::ser;
use bitgrin_p2p::types::PeerAddr;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<PeerAddr, ser::Error> = ser::deserialize(&mut d);
});
