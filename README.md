# Surgical Intelligence

A portable, model-independent engine for accumulating, maintaining, and retrieving structured knowledge from clinical and surgical experience.

**Status:** Early architecture / foundation.

## Goal

Surgical Intelligence is not another large language model and is not a replacement for a surgical registry. It is an intelligence substrate that accumulates structured knowledge over time and makes that state portable across machines, applications, and model providers.

The first local engine should:
1. Ingest observations and candidate learnings.
2. Represent assertions with provenance, confidence, and temporal validity.
3. Distinguish stable knowledge from temporary context.
4. Track revisions and contradictions without silently erasing history.
5. Retrieve relevant knowledge and explain why it matched.
6. Export and restore its complete learned state on another machine.

## Primary knowledge sources\n\nSurgical Intelligence accumulates knowledge from four first-class sources: structured surgeon learning (including original slots and later amendments), operative images and video (including frame/time-anchored annotations), papers deliberately retained by the user (including extracted and traceable evidence claims), and de-identified case/procedure/outcome data. These sources are connected in one portable knowledge graph. Text, image and video embeddings are replaceable retrieval indexes derived from those sources; they are never the canonical knowledge store.\n\nSee [Source Knowledge Model](docs/SOURCE_KNOWLEDGE_MODEL.md) for the multimodal source model and implementation sequence.\n\n## Principles

- **Model-independent core:** no LLM or hosted model is required for core state management.
- **Portable state:** knowledge is a versioned data contract, not opaque model weights.
- **Provenance first:** every assertion can identify its source and ingestion event.
- **Temporal awareness:** record when something was observed and when it is believed to be valid.
- **Explicit uncertainty:** confidence and epistemic status are first-class fields.
- **Auditability:** updates are explainable and history is retained.
- **Privacy by design:** patient-identifiable information is excluded from the default knowledge path.
- **Clinical caution:** decision support only; no autonomous diagnosis or treatment.

## Architecture

```text
Registry | Dictation | Chat | Research tools
                  |
        Ingestion / validation boundary
      parsing / de-identification / review
                  |
          Surgical Intelligence core
  +-----------------------------------------+
  | Assertions | Events | Provenance        |
  | Temporal   | Merge  | Retrieval | Export |
  +-----------------------------------------+
                  |
          Portable versioned state
                  |
      Optional LLM / embedding adapters
```

The core owns deterministic state transitions and retrieval contracts. Models may propose structured assertions or interpret queries, but model output is untrusted input and must pass validation before it changes memory.

## Workspace

- `crates/si-core`: domain types and deterministic memory operations.
- `schemas/`: versioned interchange formats.
- `python/`: dataset preparation and evaluation harness.
- `docs/`: architecture and design decisions.
- `tests/fixtures/`: synthetic, non-identifying fixtures.

## Build

Rust:
```bash
cargo test --workspace
```

Python evaluation harness:
```bash
python -m venv .venv
# Windows: .venv\\Scripts\\activate
# macOS/Linux: source .venv/bin/activate
pip install -e "python/[dev]"
pytest
```

## Roadmap

### M1 — Portable memory schema and versioning
- [x] Unified v2 state envelope for graph nodes/edges, source-artifact manifests, publications, events, embedding profiles and extensions.
- [x] Preserve the v1 assertion-memory reader and add explicit v1 memory/legacy graph migration.
- [x] Rust validation for graph references, identity, checksums, temporal ranges and source links.
- [x] CLI migration and v2 registry import; JSON Schema and implementation documentation.
- [x] Registry media manifest adapter and retained-paper/publication claim importer.
- [x] Portable media/document bundle packaging and actual byte-length/SHA-256 verification.
- [ ] Final M1 acceptance: latest CI green and Windows/macOS round-trip fixtures; trusted exporter integration remains in the source registry.

### M2 — Memory learning and update engine
- [ ] Append-only, idempotent event/update engine.
- [ ] Preserve amendments, contradictions, corroboration and supersession.
- [ ] Human review gates for clinical interpretation and reusable principles.
- [ ] Rebuildable derived summaries and embeddings.
- [ ] Python evaluation for fidelity, temporal consistency, conflict preservation and portability.

See [M1 and M2 delivery plan](docs/MILESTONES_M1_M2.md) for detailed deliverables and acceptance criteria.

## Scope boundary

The registry remains a separate system of record. Surgical Intelligence should consume explicit, permissioned, appropriately de-identified exports or events. Its core must not depend on a registry's database schema.

See [Architecture](docs/ARCHITECTURE.md), [Memory model](docs/MEMORY_MODEL.md), [Knowledge graph](docs/KNOWLEDGE_GRAPH.md), [Portability](docs/PORTABILITY.md), [CLI usage](docs/CLI.md), and [Safety & Privacy](docs/SAFETY_AND_PRIVACY.md).

## Local CLI

The first terminal interface is available through the `si` binary:

```bash
cargo run --release -p si-cli -- init --store ./data/surgical-memory.json
```

See [CLI usage](docs/CLI.md) for adding assertions, validating, exporting, and importing.
