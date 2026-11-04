# Architecture decision log

## ADR-001: Rust core, Python evaluation harness

**Status:** Accepted

**Decision:** Implement deterministic memory and portability primitives in Rust. Use Python for dataset preparation, benchmark orchestration, statistical analysis, and rapid experimentation.

**Rationale:** Rust offers a compact deployable core with explicit types and predictable runtime behavior. Python offers a mature research/evaluation ecosystem. Keeping the boundary explicit avoids making Python packages or a specific LLM provider mandatory for core operation.

## ADR-002: Portable state is explicit data, not model weights

**Status:** Accepted

**Decision:** The canonical portable asset is a versioned, inspectable state envelope containing structured assertions and event history.

**Rationale:** Opaque weights do not preserve assertion-level provenance, temporal validity, or explainable history. Learned models may later be exported as optional, separately versioned components, but they cannot be the only representation of accumulated knowledge.

## ADR-003: Registry remains a separate system of record

**Status:** Accepted

**Decision:** Integrate through adapters and explicit export/event contracts. Do not couple the core to the registry's database schema.

**Rationale:** The engine should remain portable and usable with multiple registries, dictation tools, research systems, and local workflows.

## ADR-004: Assertions carry epistemic status and provenance

**Status:** Accepted

**Decision:** Every assertion records its source type, epistemic status, observation time, optional validity interval, retention class, and lifecycle status.

**Rationale:** A surgeon's recollection, a direct observation, a literature claim, and a generated hypothesis must not become indistinguishable text in a memory store.
