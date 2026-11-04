# M1 implementation: portable memory schema and versioning

## Current implementation

M1 introduces a unified v2 portable state while retaining the original assertion-memory v1 format and reader.

- Format: `surgical-intelligence-state`
- Version: `2`
- JSON Schema: `schemas/state-v2.schema.json`
- Rust model: `crates/si-core/src/state_v2.rs`
- Migration dispatch: `crates/si-core/src/migration.rs`
- CLI: `si migrate`, `si import-registry`

The v2 envelope contains a graph (typed nodes and edges), typed My Experience learning records with original slots and amendments, source-artifact manifests, publication records and traceable evidence claims, append-only event payloads, embedding profiles and namespaced extensions. It has no mandatory database, model, vector store or object-store dependency.

## Canonical vs derived

Canonical:
- Graph nodes/edges, provenance, epistemic state, lifecycle and temporal validity.
- Source artifact identity, checksum, modality, availability, source and parent lineage, frame/time anchors.
- Publication identity and user retention decision.
- Versioned event payloads and namespaced extensions.

Derived:
- Embedding vectors, summaries, thumbnails, extracted frames, OCR/transcripts and model annotations. Derived artefacts must retain source lineage; embeddings are explicitly marked as derived caches.

Media bytes are not embedded in the JSON envelope. The manifest can state that a source is available, withheld, externally referenced or missing. A media bundle/transport and cryptographic verification of actual bytes are a later extension; the checksum field is defined now.

## Migration

Migrate a v1 assertion-memory file:

```bash
si migrate --input ./memory-v1.json --output ./state-v2.json
```

Import an authorized de-identified registry export directly to v2:

```bash
si import-registry --input ./registry-export.json --output ./state-v2.json
```

The migration path also accepts the existing v1 `surgical-intelligence-knowledge-graph` envelope. Unknown formats and versions fail closed. Output creation refuses to overwrite an existing file.

v1 assertion IDs are preserved as graph node IDs. Legacy event JSON is preserved in the v2 event collection. Original v1 files are not modified.

## Validation

Rust validation currently checks:
- Format and major version.
- Required manifest fields.
- Unique graph node and edge IDs.
- Non-empty node type/scope/label and valid epistemic/lifecycle values.
- Confidence bounds and temporal validity.
- Non-dangling edge endpoints.
- Unique source-artifact IDs, modality/availability, SHA-256 shape, frame intervals and source/parent references.
- Unique publication IDs, retention state and source artifact references.
- Embedding profile identity/dimensions and derived-cache flag.
- Namespaced extension keys.

The JSON Schema is the interchange contract. Runtime validation adds graph-wide referential checks that JSON Schema alone cannot express. Event payloads are retained as versioned JSON pending the M2 event vocabulary; their detailed semantic validation belongs to M2.

## Known M1 boundaries

- The registry exporter itself is not implemented in the source registry. Use synthetic fixtures until its privacy, purpose authorization and media-export controls are implemented and reviewed.
- Existing registry graph v1 is migrated as graph nodes/edges; source-specific provenance already present in the graph is preserved.
- Registry My Experience records are additionally mapped into typed learning records, retaining slot names, text, origin, sign tags, shown-by media type, seal time and amendments. The registry exporter still needs to supply approved media asset references before the adapter can populate actual image/video manifests.
- Media bytes are not bundled by this first implementation.
- v1 memory event payloads are preserved, but event-specific migration into a typed v2 event enum is deferred to M2.
- No neural model is trained or required to read/write v2.


## Verification

The implementation at commit `1d1cacea4289448994d3b075d7f767006fc5887f` passed the repository CI workflow:
- Rust formatting invocation: passed.
- Clippy with warnings denied: passed.
- Rust workspace tests: passed.
- Python evaluation tests: passed.

Run: https://github.com/manjunathhari/Surgical-Intelligence/actions/runs/41876115243

## Remaining integration work before calling the entire M1 milestone closed

The portable schema, core types, migration path, CLI and validation tests are implemented. The remaining work is at the source-ingestion boundary:
- Implement and review the registry-side authorized exporter, including actual image/video asset references and privacy/de-identification checks.
- Add the retained-paper ingestion adapter that populates publication records and evidence claims from the user's saved papers.
- Implement media bundle transport and verify actual bytes against manifest checksums (the current v2 validator checks checksum syntax and manifest references).
- Add cross-platform fixture runs on Windows/macOS in addition to the Linux CI runner.

These are deliberately not faked by the core: the registry remains the system of record, and patient media must not be exported until its purpose authorization and privacy workflow is in place.


## Source adapter completion

The workspace now includes the registry media manifest mapper, a retained-publication importer for bibliographic metadata and separately structured evidence claims, and portable directory bundle packaging/verification. Bundle commands verify actual file bytes against declared byte length and SHA-256. See `docs/M1_SOURCE_ADAPTERS.md` for the import contracts and commands.

The registry export remains backward-compatible: `media_assets` is optional in the existing v1 export format. A trusted exporter in the source registry must populate it only after applying the required export-purpose and authorization checks. This repository does not connect to the registry database or bypass its access controls.

The publication importer accepts a user-authorized export of already-retained papers. It does not fetch paywalled content or manufacture evidence claims from a PDF; claim review state is preserved as supplied.
