# Decision 002: Add an explicit surgical knowledge graph

- **Status:** Accepted design direction; implementation pending
- **Date:** 2026-11-04

## Context

The current assertion model captures atomic claims with provenance and lifecycle metadata, but cannot yet represent the structure of surgical experience: what happened in an episode, what was observed, which decision followed, what action was taken, and what outcome resulted.

Flattening this sequence into a paragraph or embedding loses explicit links. Automatically converting an individual case into a general rule risks overgeneralization.

## Decision

Introduce a versioned JSON knowledge-graph interchange format alongside (not instead of) the existing memory envelope. It contains typed nodes and directed, typed edges. Every node and edge carries provenance and epistemic/lifecycle metadata. Nodes distinguish case-specific episodes, reusable learning, published evidence and reference concepts.

A reusable general principle must carry an explicit human-review state and links to its source experiences and evidence. Contradictory claims remain represented; they are not overwritten.

## Consequences

- Existing assertion memory remains backward compatible.
- The graph schema can evolve independently with explicit versioning and migrations.
- Rust remains responsible for deterministic validation and state transitions.
- Model adapters may propose nodes and edges but cannot bypass validation or human review.
- JSON Schema is an interchange contract, not proof of clinical correctness or privacy.
- Graph persistence and Rust implementation are follow-on work; this decision does not claim those features already exist.
