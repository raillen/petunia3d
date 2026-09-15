//! Fuzz target: OBJ loader boundary (`tobj`, P0-13).
//!
//! Run with `cargo +nightly fuzz run obj_parse` (see `fuzz/README.md`).
//! The harness is bounded (64 KiB) and asserts the parser never panics:
//! only typed `ObjImportError`s may escape.

#![no_main]

use libfuzzer_sys::fuzz_target;

/// Max input size: hostile OBJ fixtures beyond this are truncated.
const MAX_INPUT: usize = 64 * 1024;

fuzz_target!(|data: &[u8]| {
    let bounded = if data.len() > MAX_INPUT {
        &data[..MAX_INPUT]
    } else {
        data
    };
    let _ = petunia_project::import_obj_bytes(bounded);
});
