//! Unified portable knowledge state (v2). This is canonical data, not an embedding cache.
use crate::PortableEnvelope;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};
use uuid::Uuid;

pub const STATE_FORMAT_V2: &str = "surgical-intelligence-state";
pub const STATE_VERSION_V2: u32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Temporal {
    pub observed_at: DateTime<Utc>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReviewState {
    pub state: String,
    pub reviewer_ref: Option<String>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub rationale: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphNode {
    pub id: Uuid,
    pub node_type: String,
    pub knowledge_scope: String,
    pub label: String,
    pub epistemic_status: String,
    pub confidence: Option<f32>,
    pub lifecycle_status: String,
    pub retention: String,
    pub provenance: crate::Provenance,
    pub temporal: Temporal,
    pub review: ReviewState,
    #[serde(default)]
    pub data: Value,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphEdge {
    pub id: Uuid,
    pub from: Uuid,
    pub relation: String,
    pub to: Uuid,
    pub epistemic_status: String,
    pub lifecycle_status: String,
    pub provenance: crate::Provenance,
    pub observed_at: DateTime<Utc>,
    pub confidence: Option<f32>,
    pub note: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct KnowledgeGraph {
    #[serde(default)]
    pub nodes: Vec<GraphNode>,
    #[serde(default)]
    pub edges: Vec<GraphEdge>,
}

/// Original My Experience learning, preserving every labelled slot and amendment.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearningRecord {
    pub id: Uuid,
    pub episode_node_id: Option<Uuid>,
    pub source_case_ref: Option<String>,
    pub ordinal: u8,
    pub shown_by: String,
    pub sign_tags: Vec<String>,
    pub how_showed: Option<String>,
    pub slots: Vec<LearningSlot>,
    pub amendments: Vec<LearningAmendment>,
    pub added_after_seal: bool,
    pub sealed_at: Option<DateTime<Utc>>,
    pub provenance: crate::Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearningSlot {
    pub name: String,
    pub text: String,
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LearningAmendment {
    pub slot: String,
    pub text: String,
    pub amended_at: DateTime<Utc>,
}

/// A claim is not the paper itself and is not automatically endorsed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvidenceClaim {
    pub id: Uuid,
    pub publication_id: Uuid,
    pub claim: String,
    pub locator: Option<String>,
    pub population: Option<String>,
    pub intervention: Option<String>,
    pub comparator: Option<String>,
    pub outcome: Option<String>,
    pub effect_estimate: Option<String>,
    pub uncertainty: Option<String>,
    pub limitations: Vec<String>,
    pub epistemic_status: String,
    pub review_state: String,
    pub provenance: crate::Provenance,
}

/// Manifest metadata only. Media bytes live in a separately governed object store/bundle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SourceArtifact {
    pub id: Uuid,
    pub modality: String,
    pub media_type: Option<String>,
    pub mime_type: Option<String>,
    pub sha256: Option<String>,
    pub byte_length: Option<u64>,
    pub availability: String,
    pub storage_ref: Option<String>,
    pub source_node_id: Option<Uuid>,
    pub parent_artifact_id: Option<Uuid>,
    pub frame_start_ms: Option<u64>,
    pub frame_end_ms: Option<u64>,
    pub imported_at: DateTime<Utc>,
    pub provenance: crate::Provenance,
    #[serde(default)]
    pub metadata: BTreeMap<String, Value>,
}

/// A publication is distinct from the claims extracted from it and from the user's appraisal.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicationRecord {
    pub id: Uuid,
    pub doi: Option<String>,
    pub pmid: Option<String>,
    pub pmcid: Option<String>,
    pub title: String,
    pub authors: Vec<String>,
    pub journal: Option<String>,
    pub publication_year: Option<i32>,
    pub version: Option<String>,
    pub retention_state: String,
    pub retained_by: Option<String>,
    pub decision_at: Option<DateTime<Utc>>,
    pub source_artifact_id: Option<Uuid>,
    pub provenance: crate::Provenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddingProfile {
    pub id: String,
    pub modality: String,
    pub provider: String,
    pub model: String,
    pub model_revision: Option<String>,
    pub dimensions: u32,
    pub normalization: Option<String>,
    pub preprocessing_version: String,
    pub derived_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StateManifest {
    pub minimum_reader_version: String,
    pub exporter_version: String,
    pub created_at: DateTime<Utc>,
    pub source_systems: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PortableStateV2 {
    pub format: String,
    pub version: u32,
    pub manifest: StateManifest,
    #[serde(default)]
    pub graph: KnowledgeGraph,
    #[serde(default)]
    pub source_artifacts: Vec<SourceArtifact>,
    #[serde(default)]
    pub publications: Vec<PublicationRecord>,
    #[serde(default)]
    pub learning_records: Vec<LearningRecord>,
    #[serde(default)]
    pub evidence_claims: Vec<EvidenceClaim>,
    /// Append-only domain events. Event payloads are versioned and validated by their owning subsystem.
    #[serde(default)]
    pub events: Vec<Value>,
    /// Embeddings are optional derived caches; no vector payload is canonical knowledge.
    #[serde(default)]
    pub embedding_profiles: Vec<EmbeddingProfile>,
    #[serde(default)]
    pub extensions: BTreeMap<String, Value>,
}

impl PortableStateV2 {
    pub fn empty() -> Self {
        Self {
            format: STATE_FORMAT_V2.into(),
            version: STATE_VERSION_V2,
            manifest: StateManifest {
                minimum_reader_version: "0.2.0".into(),
                exporter_version: env!("CARGO_PKG_VERSION").into(),
                created_at: Utc::now(),
                source_systems: Vec::new(),
            },
            graph: KnowledgeGraph::default(),
            source_artifacts: Vec::new(),
            publications: Vec::new(),
            learning_records: Vec::new(),
            evidence_claims: Vec::new(),
            events: Vec::new(),
            embedding_profiles: Vec::new(),
            extensions: BTreeMap::new(),
        }
    }

    pub fn from_graph_values(nodes: Vec<Value>, edges: Vec<Value>, source_system: &str, exported_at: &str) -> Result<Self, String> {
        let mut state = Self::empty();
        let parsed_at = DateTime::parse_from_rfc3339(exported_at)
            .map_err(|e| format!("invalid export timestamp: {e}"))?.with_timezone(&Utc);
        state.manifest.created_at = parsed_at;
        state.manifest.source_systems.push(source_system.to_string());
        state.graph.nodes = nodes.into_iter().map(|v| serde_json::from_value(v).map_err(|e| format!("invalid graph node: {e}"))).collect::<Result<_,_>>()?;
        state.graph.edges = edges.into_iter().map(|v| serde_json::from_value(v).map_err(|e| format!("invalid graph edge: {e}"))).collect::<Result<_,_>>()?;
        state.validate()?;
        Ok(state)
    }

    pub fn to_json(&self) -> Result<String, String> {
        self.validate()?;
        serde_json::to_string_pretty(self).map_err(|e| format!("could not serialize v2 state: {e}"))
    }

    pub fn from_json(input: &str) -> Result<Self, String> {
        let state: Self = serde_json::from_str(input).map_err(|e| format!("invalid v2 state: {e}"))?;
        if state.format != STATE_FORMAT_V2 { return Err(format!("unsupported state format: {}", state.format)); }
        if state.version != STATE_VERSION_V2 { return Err(format!("unsupported state version: {}", state.version)); }
        state.validate()?;
        Ok(state)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.format != STATE_FORMAT_V2 || self.version != STATE_VERSION_V2 {
            return Err("portable state format/version mismatch".into());
        }
        if self.manifest.minimum_reader_version.trim().is_empty() || self.manifest.exporter_version.trim().is_empty() {
            return Err("manifest reader/exporter versions must be non-empty".into());
        }
        let mut ids = HashSet::new();
        for node in &self.graph.nodes {
            if !ids.insert(node.id) { return Err(format!("duplicate graph node id {}", node.id)); }
            if node.node_type.trim().is_empty() || node.label.trim().is_empty() || node.knowledge_scope.trim().is_empty() {
                return Err(format!("node {} has empty type, scope or label", node.id));
            }
            if !matches!(node.epistemic_status.as_str(), "observed"|"reported"|"inferred"|"hypothesis") {
                return Err(format!("node {} has invalid epistemic status", node.id));
            }
            if !matches!(node.lifecycle_status.as_str(), "active"|"superseded"|"retracted"|"disputed") {
                return Err(format!("node {} has invalid lifecycle status", node.id));
            }
            if let Some(c) = node.confidence { if !c.is_finite() || !(0.0..=1.0).contains(&c) { return Err(format!("node {} has invalid confidence", node.id)); } }
            if let (Some(a),Some(b)) = (node.temporal.valid_from,node.temporal.valid_until) { if b<a { return Err(format!("node {} has invalid validity interval", node.id)); } }
        }
        let mut edge_ids = HashSet::new();
        for edge in &self.graph.edges {
            if !edge_ids.insert(edge.id) { return Err(format!("duplicate graph edge id {}", edge.id)); }
            if !ids.contains(&edge.from) || !ids.contains(&edge.to) { return Err(format!("edge {} has a dangling endpoint", edge.id)); }
            if edge.relation.trim().is_empty() { return Err(format!("edge {} has empty relation", edge.id)); }
            if let Some(c) = edge.confidence { if !c.is_finite() || !(0.0..=1.0).contains(&c) { return Err(format!("edge {} has invalid confidence", edge.id)); } }
        }
        let mut artifact_ids = HashSet::new();
        for artifact in &self.source_artifacts {
            if !artifact_ids.insert(artifact.id) { return Err(format!("duplicate source artifact id {}", artifact.id)); }
            if !matches!(artifact.modality.as_str(), "image"|"video"|"audio"|"document"|"frame"|"clip"|"other") { return Err(format!("artifact {} has invalid modality", artifact.id)); }
            if !matches!(artifact.availability.as_str(), "available"|"withheld"|"external_reference"|"missing") { return Err(format!("artifact {} has invalid availability", artifact.id)); }
            if let Some(hash) = &artifact.sha256 {
                if hash.len()!=64 || !hash.bytes().all(|b| b.is_ascii_hexdigit()) { return Err(format!("artifact {} has invalid SHA-256 checksum", artifact.id)); }
            }
            if let (Some(start),Some(end)) = (artifact.frame_start_ms,artifact.frame_end_ms) { if end < start { return Err(format!("artifact {} has invalid frame interval", artifact.id)); } }
            if let Some(parent) = artifact.parent_artifact_id { if parent == artifact.id { return Err(format!("artifact {} cannot derive from itself", artifact.id)); } }
            if let Some(node_id) = artifact.source_node_id { if !ids.contains(&node_id) { return Err(format!("artifact {} references missing graph node", artifact.id)); } }
        }
        for artifact in &self.source_artifacts {
            if let Some(parent) = artifact.parent_artifact_id { if !artifact_ids.contains(&parent) { return Err(format!("artifact {} references missing parent artifact", artifact.id)); } }
        }
        let mut learning_ids = HashSet::new();
        for learning in &self.learning_records {
            if !learning_ids.insert(learning.id) { return Err(format!("duplicate learning record id {}", learning.id)); }
            if !(1..=3).contains(&learning.ordinal) { return Err(format!("learning {} has invalid ordinal", learning.id)); }
            if learning.shown_by.trim().is_empty() { return Err(format!("learning {} has empty shown_by", learning.id)); }
            if let Some(episode) = learning.episode_node_id { if !ids.contains(&episode) { return Err(format!("learning {} references missing episode", learning.id)); } }
            let mut slot_names = HashSet::new();
            for slot in &learning.slots {
                if slot.name.trim().is_empty() || !slot_names.insert(&slot.name) { return Err(format!("learning {} has empty or duplicate slot names", learning.id)); }
                if slot.origin.trim().is_empty() { return Err(format!("learning {} has empty slot origin", learning.id)); }
            }
            if learning.amendments.windows(2).any(|w| w[1].amended_at < w[0].amended_at) { return Err(format!("learning {} amendments are not chronological", learning.id)); }
        }
        let mut publication_ids = HashSet::new();
        for paper in &self.publications {
            if !publication_ids.insert(paper.id) { return Err(format!("duplicate publication id {}", paper.id)); }
            if paper.title.trim().is_empty() { return Err(format!("publication {} has empty title", paper.id)); }
            if !matches!(paper.retention_state.as_str(), "kept"|"hidden"|"unreviewed") { return Err(format!("publication {} has invalid retention state", paper.id)); }
            if let Some(asset) = paper.source_artifact_id { if !artifact_ids.contains(&asset) { return Err(format!("publication {} references missing artifact", paper.id)); } }
        }
        let mut claim_ids = HashSet::new();
        for claim in &self.evidence_claims {
            if !claim_ids.insert(claim.id) { return Err(format!("duplicate evidence claim id {}", claim.id)); }
            if !publication_ids.contains(&claim.publication_id) { return Err(format!("evidence claim {} references missing publication", claim.id)); }
            if claim.claim.trim().is_empty() { return Err(format!("evidence claim {} has empty text", claim.id)); }
            if !matches!(claim.epistemic_status.as_str(), "observed"|"reported"|"inferred"|"hypothesis") { return Err(format!("evidence claim {} has invalid epistemic status", claim.id)); }
        }
        let mut profile_ids = HashSet::new();
        for profile in &self.embedding_profiles {
            if !profile_ids.insert(&profile.id) { return Err(format!("duplicate embedding profile {}", profile.id)); }
            if profile.id.trim().is_empty() || profile.model.trim().is_empty() || profile.dimensions == 0 || !profile.derived_cache {
                return Err(format!("embedding profile {} must be named, dimensioned and marked as derived cache", profile.id));
            }
        }
        for namespace in self.extensions.keys() {
            if !namespace.contains(':') { return Err(format!("extension namespace '{namespace}' must be namespaced (e.g. vendor:key)")); }
        }
        Ok(())
    }
}

/// Deterministic, non-destructive migration from the original assertion-memory v1 envelope.
pub fn migrate_memory_v1(input: &str) -> Result<PortableStateV2, String> {
    let old = PortableEnvelope::from_json(input)?;
    let mut state = PortableStateV2::empty();
    state.manifest.created_at = DateTime::parse_from_rfc3339(&old.exported_at)
        .map_err(|e| format!("invalid v1 export timestamp: {e}"))?.with_timezone(&Utc);
    state.manifest.source_systems.push("surgical-intelligence-memory-v1".into());
    for assertion in old.state.assertions.values() {
        let node = GraphNode {
            id: assertion.id,
            node_type: "assertion".into(),
            knowledge_scope: match assertion.retention { crate::RetentionClass::Episodic => "case_specific", _ => "reusable_learning" }.into(),
            label: assertion.statement.clone(),
            epistemic_status: format!("{:?}", assertion.epistemic_status).to_lowercase(),
            confidence: assertion.confidence,
            lifecycle_status: format!("{:?}", assertion.status).to_lowercase(),
            retention: format!("{:?}", assertion.retention).to_lowercase(),
            provenance: assertion.provenance.clone(),
            temporal: Temporal { observed_at: assertion.observed_at, valid_from: assertion.valid_from, valid_until: assertion.valid_until },
            review: ReviewState { state: "not_required".into(), reviewer_ref: None, reviewed_at: None, rationale: None },
            data: serde_json::json!({"subject":assertion.subject,"predicate":assertion.predicate,"object":assertion.object,"statement":assertion.statement}),
            tags: assertion.tags.clone(),
        };
        state.graph.nodes.push(node);
    }
    state.events = old.state.events.iter().map(|e| serde_json::to_value(e).expect("serializable memory event")).collect();
    state.validate()?;
    Ok(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Assertion, AssertionStatus, EpistemicStatus, MemoryState, Provenance, RetentionClass};
    use chrono::Utc;

    fn sample_node(id: Uuid) -> GraphNode {
        GraphNode { id, node_type:"technique".into(), knowledge_scope:"reference_concept".into(), label:"Synthetic technique".into(), epistemic_status:"reported".into(), confidence:None, lifecycle_status:"active".into(), retention:"durable".into(), provenance:Provenance{source_type:"test".into(),source_id:None,source_version:None,note:None}, temporal:Temporal{observed_at:Utc::now(),valid_from:None,valid_until:None}, review:ReviewState{state:"not_required".into(),reviewer_ref:None,reviewed_at:None,rationale:None}, data:Value::Null, tags:vec![] }
    }

    #[test]
    fn v2_round_trip_preserves_graph_and_media_manifest() {
        let mut s=PortableStateV2::empty();
        let id=Uuid::new_v4(); s.graph.nodes.push(sample_node(id));
        s.source_artifacts.push(SourceArtifact{id:Uuid::new_v4(),modality:"video".into(),media_type:Some("operative_video".into()),mime_type:Some("video/mp4".into()),sha256:Some("a".repeat(64)),byte_length:Some(123),availability:"withheld".into(),storage_ref:None,source_node_id:Some(id),parent_artifact_id:None,frame_start_ms:Some(1000),frame_end_ms:Some(2500),imported_at:Utc::now(),provenance:Provenance{source_type:"synthetic".into(),source_id:None,source_version:None,note:None},metadata:BTreeMap::new()});
        let restored=PortableStateV2::from_json(&s.to_json().unwrap()).unwrap();
        assert_eq!(restored,s);
    }

    #[test]
    fn rejects_dangling_edges_and_bad_checksums() {
        let mut s=PortableStateV2::empty();
        let a=Uuid::new_v4(); let b=Uuid::new_v4();
        s.graph.nodes.push(sample_node(a));
        s.graph.edges.push(GraphEdge{id:Uuid::new_v4(),from:a,to:b,relation:"supports".into(),epistemic_status:"reported".into(),lifecycle_status:"active".into(),provenance:Provenance{source_type:"test".into(),source_id:None,source_version:None,note:None},observed_at:Utc::now(),confidence:None,note:None});
        assert!(s.validate().is_err());
        s.graph.edges.clear();
        s.source_artifacts.push(SourceArtifact{id:Uuid::new_v4(),modality:"image".into(),media_type:None,mime_type:None,sha256:Some("not-a-hash".into()),byte_length:None,availability:"missing".into(),storage_ref:None,source_node_id:None,parent_artifact_id:None,frame_start_ms:None,frame_end_ms:None,imported_at:Utc::now(),provenance:Provenance{source_type:"test".into(),source_id:None,source_version:None,note:None},metadata:BTreeMap::new()});
        assert!(s.validate().is_err());
    }

    #[test]
    fn migrates_v1_assertions_without_losing_identity_or_events() {
        let mut memory=MemoryState::new();
        let a=Assertion{id:Uuid::new_v4(),subject:"procedure:x".into(),predicate:"has_step".into(),object:"step".into(),statement:"Synthetic assertion".into(),epistemic_status:EpistemicStatus::Reported,confidence:Some(0.7),retention:RetentionClass::Durable,status:AssertionStatus::Active,provenance:Provenance{source_type:"test".into(),source_id:Some("s1".into()),source_version:None,note:None},observed_at:Utc::now(),valid_from:None,valid_until:None,tags:vec![]};
        let id=a.id; memory.add_assertion(a).unwrap();
        let old=PortableEnvelope::new(memory); let migrated=migrate_memory_v1(&old.to_json().unwrap()).unwrap();
        assert_eq!(migrated.graph.nodes[0].id,id); assert_eq!(migrated.events.len(),1);
    }
}
