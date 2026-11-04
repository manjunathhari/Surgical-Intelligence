# M1 and M2 delivery plan

**Decision:** M1 and M2 are the next two engineering milestones. The current Rust core, portable assertion memory, knowledge-graph contract, registry adapter and Python evaluation harness remain in place. Do not start model training as part of either milestone.

## M1 — Portable memory schema and versioning

### Objective
Make all accumulated knowledge portable, inspectable, versioned and restorable independently of its originating application, database, operating system, model or embedding provider.

### Canonical state
The portable state must preserve:
- Assertions and typed knowledge-graph nodes/edges.
- Source records and immutable source references: surgical learning, case episode, publication, image, video and media segment.
- Original learning slot values, slot origins, sealed baseline and append-only amendments.
- Publication identity, retained/hidden decision, extracted claims and exact source locators.
- Media manifests, checksums, frame/time anchors, annotation provenance and derivative lineage.
- Events, provenance, epistemic status, confidence semantics, lifecycle and temporal validity.
- Review/promotion state and links to supporting and contradictory evidence.

### Versioning policy
- Keep 'surgical-intelligence-memory' v1 readable; never reinterpret its bytes under a new schema.
- Introduce a new major portable envelope version for the unified graph/source state (proposed 'surgical-intelligence-state' v2).
- Include format, schema version, minimum reader version, exporter version, export timestamp and source manifest.
- Add explicit v1-to-v2 migration; migration must be deterministic, tested, and non-destructive. Preserve original IDs where possible and record migration provenance.
- Unknown major versions fail closed. Unknown optional extensions may be retained as opaque extension data if their namespace is declared.
- Use stable UUIDs for graph identity and content hashes for immutable source artefacts. Do not use model-generated IDs as identity.
- Define compatibility rules and fixtures for forward/backward readers. A v2 reader may import v1 through migration; a v1 reader must never silently consume v2.
- Embedding indexes are optional, versioned caches. The export must remain useful if every vector is deleted.

### M1 deliverables
1. Normative JSON Schema for the unified v2 state.
2. Rust types and validation for envelope, nodes, edges, source manifests and extension namespaces.
3. Migration module from memory v1 and existing registry graph export to v2.
4. CLI: migrate, validate, export, import, inspect.
5. Synthetic fixtures for empty state, assertion memory, registry learning, paper, image, video segment, contradiction and migration.
6. Cross-platform JSON round-trip and checksum verification tests.
7. Documentation of schema evolution and migration guarantees.

### M1 acceptance criteria
- Export then import produces semantically identical canonical state, preserving IDs, provenance, event order, timestamps and source locators.
- v1 data migrates without dropping assertions/events.
- Invalid references, duplicate IDs, dangling graph edges, invalid timestamps, malformed checksums and unsupported versions are rejected.
- A state can be inspected without loading an LLM, vector database or Supabase.
- Media can be represented as a verified manifest even when bytes are intentionally withheld; missing media is explicit, not silently treated as present.
- No direct patient identifiers or raw clinical media are introduced into default fixtures.

## M2 — Memory learning and update engine

### Objective
Provide deterministic, auditable rules for how new information enters, relates to and changes existing knowledge. Models may propose candidate changes, but only the engine applies validated operations.

### Update pipeline
1. **Ingest:** receive a source record or structured candidate from an adapter.
2. **Normalize:** preserve original source text/slots and create typed candidate nodes/claims without flattening the source.
3. **Validate:** check schema, provenance, privacy status, temporal consistency and required fields.
4. **Link:** resolve known entity/concept references and attach explicit relationships. Uncertain links remain candidate links.
5. **Compare:** identify exact duplicates, near-duplicates, corroboration, contradiction and possible supersession. Similarity is a retrieval signal, not proof of equivalence or contradiction.
6. **Propose:** create a change set with before/after references, rationale, source evidence and uncertainty.
7. **Review:** apply automatic deterministic updates only for safe mechanical cases; require human approval for clinical interpretation, conflict resolution and promotion to reusable principles.
8. **Commit:** append immutable events; never erase the prior state.
9. **Evaluate:** record whether the update improved source fidelity, retrieval, consistency and downstream use.

### Required update semantics
- Add new knowledge without overwriting earlier observations.
- Amend a surgeon learning by preserving baseline and each amendment as a time-ordered event.
- Mark earlier assertions as superseded only when an explicit relation and rationale exist.
- Preserve contradictory claims and evidence side by side.
- Distinguish corroboration from duplication.
- Track scope and applicability: procedure, approach, anatomy, pathology, patient/context factors, surgeon/experience context where permitted.
- Keep observed, reported, inferred and hypothetical information distinct.
- Keep a candidate learning separate from an approved reusable principle.
- Allow retraction while retaining the audit trail.
- Make updates idempotent using source event IDs/content hashes; replay must not create duplicate state.
- Support late-arriving outcomes and follow-up without rewriting what the surgeon knew at the time of the original decision.
- Keep derived summaries and embeddings rebuildable from canonical records.

### Learning is not just parameter training
The first learning engine should learn by maintaining structured relationships and evidence across episodes. It should be able to answer:
- What did the surgeon observe?
- What did they expect?
- What decision did they make, and why?
- What action was performed?
- What happened next?
- What did they later change or learn?
- Which other cases support or contradict this pattern?
- Which papers and visual examples are relevant?
- Under what contexts does this learning apply, and where should it not be generalized?

A small neural model may later assist extraction, ranking or prediction. Its output is a proposal with model/version provenance, not canonical memory. Training or updating weights must never be the only record of a learning event.

### M2 deliverables
1. Rust update-engine API with typed commands and validated change sets.
2. Append-only event types for source ingestion, learning amendment, relationship addition, corroboration, contradiction, supersession, retraction, review and principle promotion.
3. Idempotency keys and deterministic replay.
4. Conflict and duplicate candidate detection interfaces (initially deterministic rules; embeddings optional).
5. Human-review queue and approval/rejection records.
6. Retrieval context builder that can assemble case + learning + media + paper evidence with source citations/locators.
7. Python evaluation harness for extraction fidelity, update correctness, conflict preservation, temporal reasoning, retrieval and replay determinism.

### M2 acceptance criteria
- Replaying the same source event does not duplicate knowledge.
- Conflicting sources are both preserved and discoverable.
- No single case automatically becomes a general surgical rule.
- Late outcomes append to the episode timeline without changing historical decision context.
- Every mutation has actor/source, timestamp, reason and before/after trace.
- Machine-generated candidates cannot bypass validation or required review.
- The same event stream produces the same canonical state on supported platforms.
- Evaluation demonstrates that updates preserve original source fidelity and do not silently erase disagreement.

## Dependencies and explicit non-goals

M1 precedes M2 because updates need stable identities, event contracts and migration semantics.

Neither milestone includes:
- Training a new neural network as the primary memory store.
- Replacing the registry or directly connecting the core to its production database.
- Treating embeddings as canonical knowledge.
- Automatically converting one surgeon's anecdote into clinical guidance.
- Unrestricted ingestion of identifiable patient media or records.

## Suggested implementation order

1. Finalize M1 v2 envelope and migration rules.
2. Implement Rust schema/types/validation and fixtures.
3. Implement migration and CLI.
4. Complete M1 round-trip, portability and corruption tests.
5. Define M2 event vocabulary and state-transition invariants.
6. Implement idempotent update commands and append-only event log.
7. Add contradiction/corroboration candidates and review gates.
8. Add evaluation benchmarks before enabling model-assisted extraction.
