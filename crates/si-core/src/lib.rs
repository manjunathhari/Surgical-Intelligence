//! Core domain model for Surgical Intelligence.
//!
//! No network, database, LLM, or embedding-provider dependency belongs here.
//! Candidate assertions from external models must be validated at an application
//! boundary before they enter this state.

mod memory;
mod migration;
mod portability;
mod state_v2;

pub use memory::{
    Assertion, AssertionStatus, EpistemicStatus, MemoryEvent, MemoryState, Provenance,
    RetentionClass,
};
pub use portability::{PortableEnvelope, PORTABLE_FORMAT, PORTABLE_VERSION};

pub use migration::migrate_to_v2;
pub use state_v2::{
    migrate_memory_v1, EmbeddingProfile, GraphEdge, GraphNode, KnowledgeGraph as PortableKnowledgeGraph,
    PortableStateV2, PublicationRecord, ReviewState, SourceArtifact, StateManifest, Temporal,
    STATE_FORMAT_V2, STATE_VERSION_V2,
};
