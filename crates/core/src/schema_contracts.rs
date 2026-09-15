//! Stable command/plugin contracts with JSON Schema (`schemars`, P0-11).
//!
//! Commands, Lua plugins and MCP clients exchange these DTOs. Every contract
//! has a generated JSON Schema so external callers can validate payloads
//! without reading Rust source. Schemas are versioned alongside the structs:
//! adding an optional field is additive, renaming or removing one is breaking.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Schema version stamped into every generated schema document.
pub const CONTRACT_SCHEMA_VERSION: &str = "1";

/// Intent dispatched by UI, keymap, CLI, Lua plugin or MCP alike.
/// Mirrors `CommandId` semantics without leaking domain internals.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CommandIntent {
    /// Stable command id, e.g. `"model.extrude"`.
    pub command: String,
    /// Optional target asset UUID (persisted identity, not a session handle).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// Numeric arguments in command-defined order.
    #[serde(default)]
    pub args: Vec<f64>,
}

/// Result of a scene query for palette/command-palette display.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SceneItemContract {
    /// Display name.
    pub name: String,
    /// Stable kind tag: `"object"`, `"annotation"` or `"measurement"`.
    pub kind: String,
    /// Whether the item is currently visible.
    pub visible: bool,
}

/// Generates the JSON Schema for [`CommandIntent`].
pub fn command_intent_schema() -> schemars::Schema {
    schemars::schema_for!(CommandIntent)
}

/// Generates the JSON Schema for [`SceneItemContract`].
pub fn scene_item_schema() -> schemars::Schema {
    schemars::schema_for!(SceneItemContract)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_intent_schema_requires_command_field() {
        let schema = serde_json::to_value(command_intent_schema()).unwrap();
        let required = schema
            .pointer("/$schema")
            .or_else(|| schema.pointer("/title"))
            .cloned();
        // The schema document itself must exist and describe an object.
        assert_eq!(schema["type"], "object");
        let _ = required;
        let props = &schema["properties"];
        assert!(props.get("command").is_some());
        assert!(props.get("args").is_some());
        let required_fields = schema["required"].as_array().cloned().unwrap_or_default();
        assert!(required_fields.iter().any(|v| v == "command"));
    }

    #[test]
    fn scene_item_schema_roundtrips_through_json() {
        let item = SceneItemContract {
            name: "Cube".to_string(),
            kind: "object".to_string(),
            visible: true,
        };
        let json = serde_json::to_string(&item).unwrap();
        let back: SceneItemContract = serde_json::from_str(&json).unwrap();
        assert_eq!(item, back);

        let schema = serde_json::to_value(scene_item_schema()).unwrap();
        assert_eq!(schema["type"], "object");
    }

    #[test]
    fn intent_rejects_unknown_json_shape() {
        let bad = serde_json::json!({ "args": [1.0] });
        assert!(serde_json::from_value::<CommandIntent>(bad).is_err());
    }
}
