//! Explicit migration entry points for portable state formats.
use crate::{migrate_memory_v1, PortableStateV2};

pub fn migrate_to_v2(input: &str) -> Result<PortableStateV2, String> {
    let value: serde_json::Value = serde_json::from_str(input).map_err(|e| format!("invalid input JSON: {e}"))?;
    match value.get("format").and_then(|v| v.as_str()) {
        Some("surgical-intelligence-memory") => migrate_memory_v1(input),
        Some("surgical-intelligence-state") => PortableStateV2::from_json(input),
        Some(other) => Err(format!("no migration path for format '{other}'")),
        None => Err("input has no format identifier".into()),
    }
}
