//! Fuzz target: `.petunia` project parser (`postcard`, P0-13).
//!
//! Run with `cargo +nightly fuzz run project_parse` (see `fuzz/README.md`).
//! Bounded (256 KiB); asserts byte-level parsing never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;

/// Max input size for project blobs.
const MAX_INPUT: usize = 256 * 1024;

fuzz_target!(|data: &[u8]| {
    let bounded = if data.len() > MAX_INPUT {
        &data[..MAX_INPUT]
    } else {
        data
    };
    let _ = petunia_project::format::load_bytes(bounded);
});
