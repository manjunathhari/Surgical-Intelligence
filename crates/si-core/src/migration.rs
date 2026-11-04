//! Explicit migration entry points for portable state formats.
use crate::{migrate_memory_v1, PortableStateV2};
use serde_json::Value;

pub fn migrate_to_v2(input: &str) -> Result<PortableStateV2, String> {
    let value: Value = serde_json::from_str(input).map_err(|e| format!("invalid input JSON: {e}"))?;
    match value.get("format").and_then(Value::as_str) {
        Some("surgical-intelligence-memory") => migrate_memory_v1(input),
        Some("surgical-intelligence-knowledge-graph") => {
            let version = value.get("version").and_then(Value::as_u64)
                .ok_or_else(|| "legacy graph has no integer version".to_string())?;
            if version != 1 { return Err(format!("unsupported legacy graph version: {version}")); }
            let exported_at = value.get("exported_at").and_then(Value::as_str)
                .ok_or_else(|| "legacy graph has no exported_at timestamp".to_string())?;
            let graph = value.get("graph").ok_or_else(|| "legacy graph has no graph object".to_string())?;
            let nodes = graph.get("nodes").and_then(Value::as_array)
                .ok_or_else(|| "legacy graph nodes must be an array".to_string())?.clone();
            let edges = graph.get("edges").and_then(Value::as_array)
                .ok_or_else(|| "legacy graph edges must be an array".to_string())?.clone();
            PortableStateV2::from_graph_values(nodes, edges, "surgical-intelligence-knowledge-graph-v1", exported_at)
        }
        Some("surgical-intelligence-state") => PortableStateV2::from_json(input),
        Some(other) => Err(format!("no migration path for format '{other}'")),
        None => Err("input has no format identifier".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_format() {
        assert!(migrate_to_v2(r#"{"format":"unknown","version":1}"#).is_err());
    }

    #[test]
    fn migrates_legacy_registry_graph_envelope() {
        let input = r#"{"format":"surgical-intelligence-knowledge-graph","version":1,"exported_at":"2026-11-04T00:00:00Z","graph":{"nodes":[],"edges":[]}}"#;
        let migrated = migrate_to_v2(input).unwrap();
        assert_eq!(migrated.version, 2);
        assert_eq!(
            migrated.manifest.source_systems,
            vec!["surgical-intelligence-knowledge-graph-v1"]
        );
    }

    #[test]
    fn rejects_unknown_legacy_graph_version() {
        let input = r#"{"format":"surgical-intelligence-knowledge-graph","version":9,"exported_at":"2026-11-04T00:00:00Z","graph":{"nodes":[],"edges":[]}}"#;
        assert!(migrate_to_v2(input).is_err());
    }
}
