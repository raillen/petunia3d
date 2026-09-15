//! Stable structured-output snapshots (`insta`, P1-14).
//!
//! Only deterministic outputs: command JSON Schemas and diagnostic event
//! shapes. Never volatile noise (timestamps, frame stats, UUIDs).

use petunia_core::{DiagnosticCategory, DiagnosticEvent, command_intent_schema, scene_item_schema};

#[test]
fn command_intent_schema_snapshot() {
    let schema = serde_json::to_value(command_intent_schema()).unwrap();
    insta::assert_json_snapshot!(schema);
}

#[test]
fn scene_item_schema_snapshot() {
    let schema = serde_json::to_value(scene_item_schema()).unwrap();
    insta::assert_json_snapshot!(schema);
}

#[test]
fn diagnostic_event_shape_snapshot() {
    let event = DiagnosticEvent::new(
        DiagnosticCategory::Commands,
        "test.snapshot",
        "snapshot shape",
    );
    insta::assert_json_snapshot!(serde_json::to_value(&event).unwrap());
}
