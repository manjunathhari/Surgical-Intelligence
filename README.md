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

## Principles

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

### M0 — Foundation
- [x] Define boundaries and principles.
- [x] Establish Rust workspace and initial domain model.
- [x] Define versioned portable state envelope.
- [x] Add deterministic in-memory insertion and export/import.
- [ ] Add merge and contradiction-resolution policies.
- [x] Define portable surgical knowledge-graph interchange schema.
- [ ] Implement Rust graph types, integrity validation, and round-trip tests.

### M1 — Memory engine
- Add source provenance and event history.
- Track updates, supersession, and temporal validity.
- Add explainable lexical and metadata retrieval.

### M2 — Evaluation
- Synthetic and de-identified benchmarks.
- Measure ingestion fidelity, retrieval relevance, update correctness, and portability.
- Test performance and state growth.

### M3 — Adapters
- Registry adapter.
- Dictation/conversation adapter.
- Optional embedding and LLM adapters.

## Scope boundary

The registry remains a separate system of record. Surgical Intelligence should consume explicit, permissioned, appropriately de-identified exports or events. Its core must not depend on a registry's database schema.

See [Architecture](docs/ARCHITECTURE.md), [Memory model](docs/MEMORY_MODEL.md), [Knowledge graph](docs/KNOWLEDGE_GRAPH.md), [Portability](docs/PORTABILITY.md), [CLI usage](docs/CLI.md), and [Safety & Privacy](docs/SAFETY_AND_PRIVACY.md).

## Local CLI

The first terminal interface is available through the `si` binary:

```bash
cargo run --release -p si-cli -- init --store ./data/surgical-memory.json
```

See [CLI usage](docs/CLI.md) for adding assertions, validating, exporting, and importing.
