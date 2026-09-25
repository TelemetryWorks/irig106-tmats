// fuzz/fuzz_targets/fuzz_parse.rs
//
// # Fuzz Target: ASCII Parser
//
// Feeds arbitrary bytes into the TMATS parser to find panics,
// buffer overflows, and infinite loops.
//
// Run: cargo +nightly fuzz run fuzz_parse
//
// ## Traceability:
//   L3-TEST-004: Fuzz testing via cargo-fuzz

#![no_main]

use libfuzzer_sys::fuzz_target;
use irig106_tmats::parse::{parse_with_options, ParseOptions, ParseMode};

fuzz_target!(|data: &[u8]| {
    // Strict mode — should not panic on any input
    let _ = parse_with_options(data, &ParseOptions {
        mode: ParseMode::Strict,
        max_attributes: 10_000,
        max_value_size: 100_000,
        ..Default::default()
    });

    // Lenient mode — should not panic on any input
    let _ = parse_with_options(data, &ParseOptions {
        mode: ParseMode::Lenient,
        max_attributes: 10_000,
        max_value_size: 100_000,
        ..Default::default()
    });
});
