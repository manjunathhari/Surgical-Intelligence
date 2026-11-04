# M1 implementation: portable memory schema and versioning

## Current implementation

M1 introduces a unified v2 portable state while retaining the original assertion-memory v1 format and reader.

- Format: `surgical-intelligence-state`
- Version: `2`
- JSON Schema: `schemas/state-v2.schema.json`
- Rust model: `crates/si-core/src/state_v2.rs`
- Migration dispatch: `crates/si-core/src/migration.rs`
- CLI: `si migrate`, `si import-registry`

The v2 envelope contains a graph (typed nodes and edges), source-artifact manifests, publication records, append-only event payloads, embedding profiles and namespaced extensions. It has no mandatory database, model, vector store or object-store dependency.

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
- Existing My Experience learning data remains in its structured graph payload, but richer first-class typed learning/publication/media ingestion adapters are future M1 completion work.
- Media bytes are not bundled by this first implementation.
- v1 memory event payloads are preserved, but event-specific migration into a typed v2 event enum is deferred to M2.
- No neural model is trained or required to read/write v2.
