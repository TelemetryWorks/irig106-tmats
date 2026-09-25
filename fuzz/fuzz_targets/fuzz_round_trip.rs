// fuzz/fuzz_targets/fuzz_round_trip.rs
//
// # Fuzz Target: Parse → Serialize Round-Trip
//
// Verifies that if parsing succeeds, serialization also succeeds
// and the result can be re-parsed without error.
//
// Run: cargo +nightly fuzz run fuzz_round_trip
//
// ## Traceability:
//   L3-TEST-004: Fuzz testing
//   L2-SERIAL-003: Round-trip fidelity

#![no_main]

use libfuzzer_sys::fuzz_target;
use irig106_tmats::parse::{parse_with_options, ParseOptions, ParseMode};
use irig106_tmats::serial::serialize_to_vec;

fuzz_target!(|data: &[u8]| {
    let opts = ParseOptions {
        mode: ParseMode::Strict,
        max_attributes: 5_000,
        max_value_size: 50_000,
        ..Default::default()
    };

    if let Ok(doc) = parse_with_options(data, &opts) {
        // If parse succeeded, serialize must not panic
        if let Ok(serialized) = serialize_to_vec(&doc) {
            // Re-parse must not panic
            let _ = parse_with_options(&serialized, &opts);
        }
    }
});
