#![no_main]
extern crate bitgrin_core;
#[macro_use]
extern crate libfuzzer_sys;

use bitgrin_core::core::transaction;
use bitgrin_core::ser;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<transaction::Transaction, ser::Error> = ser::deserialize(&mut d);
});
