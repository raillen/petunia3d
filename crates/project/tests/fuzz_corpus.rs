//! Stable mirror of the `cargo-fuzz` harnesses (P0-13).
//!
//! Feeds a bounded hostile corpus through the exact entry points the fuzz
//! targets use (`import_obj_bytes`, `format::load_bytes`) and asserts the
//! parsers never panic — only typed errors escape. Runs on stable CI; the
//! `fuzz/` targets explore the same boundaries on nightly.

use petunia_project::{format, import_obj_bytes};

/// Hostile corpus: truncated, overlong, null-injected and magic-mangled docs.
fn hostile_inputs() -> Vec<Vec<u8>> {
    let mut corpus = vec![
        vec![],
        vec![0u8; 8],
        b"\x00\x01\x02\x03\xff\xfe\xfd".to_vec(),
        b"o ".to_vec(),
        b"v 1 2\nf 1 2 3\n".to_vec(),
        b"v NaN NaN NaN\nf 1 1 1\n".to_vec(),
        "v 1 1 1\n".repeat(10_000).into_bytes(),
        b"PETUNIA\0".to_vec(),
        b"PETUNIA\0\xff\xff\xff".to_vec(),
        vec![
            0x50, 0x45, 0x54, 0x55, 0x4e, 0x49, 0x41, 0x00, 0xff, 0xff, 0xff, 0xff,
        ],
    ];
    // Valid doc with every byte position flipped once (first 64 bytes).
    let mut valid = valid_project_bytes();
    for i in 0..valid.len().min(64) {
        let mut mutated = valid.clone();
        mutated[i] ^= 0xff;
        corpus.push(mutated);
    }
    valid.truncate(7);
    corpus.push(valid);
    corpus
}

fn valid_project_bytes() -> Vec<u8> {
    let mut project = petunia_project::Project::new();
    project.add("Cube", petunia_mesh::Mesh::cube(1.0));
    let dir = std::env::temp_dir().join(format!("petunia-fuzz-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("seed.petunia");
    petunia_project::format::save(&project, &path).unwrap();
    std::fs::read(&path).unwrap()
}

#[test]
fn obj_parser_never_panics_on_hostile_corpus() {
    for input in hostile_inputs() {
        let bounded = if input.len() > 64 * 1024 {
            &input[..64 * 1024]
        } else {
            &input[..]
        };
        let _ = import_obj_bytes(bounded);
    }
}

#[test]
fn project_parser_never_panics_on_hostile_corpus() {
    for input in hostile_inputs() {
        let bounded = if input.len() > 256 * 1024 {
            &input[..256 * 1024]
        } else {
            &input[..]
        };
        let _ = format::load_bytes(bounded);
    }
}

#[test]
fn valid_project_survives_parse() {
    let bytes = valid_project_bytes();
    let project = format::load_bytes(&bytes).unwrap();
    assert!(project.assets.iter().any(|a| a.name == "Cube"));
}
