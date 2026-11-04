use crate::MemoryState;
use serde::{Deserialize, Serialize};

pub const PORTABLE_FORMAT: &str = "surgical-intelligence-memory";
pub const PORTABLE_VERSION: u32 = 1;

/// Stable interchange envelope. Do not use Rust-specific binary layouts as the
/// sole persisted representation; this JSON envelope is deliberately inspectable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortableEnvelope {
    pub format: String,
    pub version: u32,
    pub exported_at: String,
    pub state: MemoryState,
}

impl PortableEnvelope {
    pub fn new(state: MemoryState) -> Self {
        Self {
            format: PORTABLE_FORMAT.to_string(),
            version: PORTABLE_VERSION,
            exported_at: chrono::Utc::now().to_rfc3339(),
            state,
        }
    }

    pub fn to_json(&self) -> Result<String, String> {
        self.state.validate()?;
        serde_json::to_string_pretty(self)
            .map_err(|err| format!("could not serialize portable envelope: {err}"))
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let envelope: Self = serde_json::from_str(input)
            .map_err(|err| format!("invalid portable envelope: {err}"))?;
        if envelope.format != PORTABLE_FORMAT {
            return Err(format!("unsupported format: {}", envelope.format));
        }
        if envelope.version != PORTABLE_VERSION {
            return Err(format!(
                "unsupported portable version: {}",
                envelope.version
            ));
        }
        chrono::DateTime::parse_from_rfc3339(&envelope.exported_at)
            .map_err(|err| format!("invalid exported_at timestamp: {err}"))?;
        envelope.state.validate()?;
        Ok(envelope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_state_round_trips() {
        let envelope = PortableEnvelope::new(MemoryState::new());
        let json = envelope.to_json().unwrap();
        let restored = PortableEnvelope::from_json(&json).unwrap();
        assert_eq!(restored.format, PORTABLE_FORMAT);
        assert_eq!(restored.version, PORTABLE_VERSION);
        assert_eq!(restored.state, MemoryState::new());
    }

    #[test]
    fn rejects_unknown_version() {
        let input = r#"{"format":"surgical-intelligence-memory","version":99,"exported_at":"2026-01-01T00:00:00Z","state":{"assertions":{},"events":[]}}"#;
        assert!(PortableEnvelope::from_json(input).is_err());
    }

    #[test]
    fn rejects_state_with_dangling_event() {
        let input = r#"{"format":"surgical-intelligence-memory","version":1,"exported_at":"2026-01-01T00:00:00Z","state":{"assertions":{},"events":[{"event_type":"assertion_added","event_id":"d9428888-122b-4f45-82d2-b6a64f99c243","assertion_id":"7d444840-9dc0-4a47-a99d-6d9f834f45c8","occurred_at":"2026-01-01T00:00:00Z"}]}}"#;
        assert!(PortableEnvelope::from_json(input).is_err());
    }
}
