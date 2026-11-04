use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// How strongly an assertion is grounded. This is not a calibrated probability.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EpistemicStatus {
    Observed,
    Reported,
    Inferred,
    Hypothesis,
}

/// Lifecycle is explicit; a new assertion does not silently overwrite an old one.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AssertionStatus {
    Active,
    Superseded,
    Retracted,
    Disputed,
}

/// Retention class distinguishes durable learning from short-lived context.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    Durable,
    Episodic,
    Temporary,
}

/// Source reference should identify a source without embedding patient identifiers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Provenance {
    pub source_type: String,
    pub source_id: Option<String>,
    pub source_version: Option<String>,
    pub note: Option<String>,
}

/// A normalized assertion. The initial implementation uses a subject-predicate-object
/// shape while retaining a human-readable statement for straightforward retrieval.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Assertion {
    pub id: Uuid,
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub statement: String,
    pub epistemic_status: EpistemicStatus,
    /// An explicitly supplied confidence score in [0, 1], not assumed calibrated.
    pub confidence: Option<f32>,
    pub retention: RetentionClass,
    pub status: AssertionStatus,
    pub provenance: Provenance,
    pub observed_at: DateTime<Utc>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

impl Assertion {
    pub fn validate(&self) -> Result<(), String> {
        for (field, value) in [
            ("subject", self.subject.as_str()),
            ("predicate", self.predicate.as_str()),
            ("object", self.object.as_str()),
            ("statement", self.statement.as_str()),
            ("provenance.source_type", self.provenance.source_type.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(format!("{field} must not be empty"));
            }
        }
        if let Some(score) = self.confidence {
            if !score.is_finite() || !(0.0..=1.0).contains(&score) {
                return Err("confidence must be a finite number between 0 and 1".into());
            }
        }
        if let (Some(from), Some(until)) = (self.valid_from, self.valid_until) {
            if until < from {
                return Err("valid_until must not precede valid_from".into());
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "event_type", rename_all = "snake_case")]
pub enum MemoryEvent {
    AssertionAdded {
        event_id: Uuid,
        assertion_id: Uuid,
        occurred_at: DateTime<Utc>,
    },
    AssertionStatusChanged {
        event_id: Uuid,
        assertion_id: Uuid,
        previous: AssertionStatus,
        current: AssertionStatus,
        reason: String,
        occurred_at: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct MemoryState {
    pub assertions: BTreeMap<Uuid, Assertion>,
    pub events: Vec<MemoryEvent>,
}

impl MemoryState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_assertion(&mut self, assertion: Assertion) -> Result<(), String> {
        assertion.validate()?;
        if self.assertions.contains_key(&assertion.id) {
            return Err(format!("assertion {} already exists", assertion.id));
        }
        let id = assertion.id;
        let occurred_at = Utc::now();
        self.assertions.insert(id, assertion);
        self.events.push(MemoryEvent::AssertionAdded {
            event_id: Uuid::new_v4(),
            assertion_id: id,
            occurred_at,
        });
        Ok(())
    }

    pub fn change_status(
        &mut self,
        id: Uuid,
        new_status: AssertionStatus,
        reason: impl Into<String>,
    ) -> Result<(), String> {
        let assertion = self.assertions.get_mut(&id)
            .ok_or_else(|| format!("assertion {id} not found"))?;
        let previous = assertion.status;
        if previous == new_status {
            return Ok(());
        }
        let reason = reason.into();
        if reason.trim().is_empty() {
            return Err("status-change reason must not be empty".into());
        }
        assertion.status = new_status;
        self.events.push(MemoryEvent::AssertionStatusChanged {
            event_id: Uuid::new_v4(),
            assertion_id: id,
            previous,
            current: new_status,
            reason,
            occurred_at: Utc::now(),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn assertion() -> Assertion {
        Assertion {
            id: Uuid::new_v4(),
            subject: "procedure:example".into(),
            predicate: "has_consideration".into(),
            object: "example consideration".into(),
            statement: "An example synthetic learning.".into(),
            epistemic_status: EpistemicStatus::Reported,
            confidence: Some(0.8),
            retention: RetentionClass::Durable,
            status: AssertionStatus::Active,
            provenance: Provenance {
                source_type: "synthetic_test".into(),
                source_id: Some("fixture-001".into()),
                source_version: None,
                note: None,
            },
            observed_at: Utc::now(),
            valid_from: None,
            valid_until: None,
            tags: vec!["synthetic".into()],
        }
    }

    #[test]
    fn accepts_valid_assertion_and_records_event() {
        let mut state = MemoryState::new();
        state.add_assertion(assertion()).unwrap();
        assert_eq!(state.assertions.len(), 1);
        assert_eq!(state.events.len(), 1);
    }

    #[test]
    fn rejects_invalid_confidence() {
        let mut item = assertion();
        item.confidence = Some(1.5);
        assert!(item.validate().is_err());
    }

    #[test]
    fn status_change_is_audited() {
        let mut state = MemoryState::new();
        let item = assertion();
        let id = item.id;
        state.add_assertion(item).unwrap();
        state.change_status(id, AssertionStatus::Disputed, "conflicting evidence").unwrap();
        assert_eq!(state.assertions[&id].status, AssertionStatus::Disputed);
        assert_eq!(state.events.len(), 2);
    }
}
