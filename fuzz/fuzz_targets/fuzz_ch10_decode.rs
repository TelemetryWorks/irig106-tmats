// fuzz/fuzz_targets/fuzz_ch10_decode.rs
//
// # Fuzz Target: Ch10 Setup Record Payload Decode
//
// Feeds arbitrary bytes as a Type 0x01 setup record payload.
// Must not panic regardless of input.
//
// Run: cargo +nightly fuzz run fuzz_ch10_decode
//
// ## Traceability:
//   L3-TEST-004: Fuzz testing
//   L2-CH10-001, L2-CH10-003

#![no_main]

use libfuzzer_sys::fuzz_target;
use irig106_tmats::ch10::decode_setup_payload;

fuzz_target!(|data: &[u8]| {
    // Must not panic on any input
    let _ = decode_setup_payload(data);
});
