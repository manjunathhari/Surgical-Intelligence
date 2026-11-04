//! Core domain model for Surgical Intelligence.
//!
//! No network, database, LLM, or embedding-provider dependency belongs here.
//! Candidate assertions from external models must be validated at an application
//! boundary before they enter this state.

mod memory;
mod portability;

pub use memory::{
    Assertion, AssertionStatus, EpistemicStatus, MemoryEvent, MemoryState,
    Provenance, RetentionClass,
};
pub use portability::{PortableEnvelope, PORTABLE_FORMAT, PORTABLE_VERSION};
