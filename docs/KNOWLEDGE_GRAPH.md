# Portable surgical knowledge graph

## Purpose

The existing assertion model remains the smallest unit of a claim. The graph model adds explicit surgical episodes, clinical events, decisions, actions, outcomes, reflections, evidence claims and reusable concepts, connected by typed relationships.

The graph preserves **how knowledge arose**, not merely a summary of what happened.

## Knowledge scopes

- **Case-specific:** what was observed, decided, performed and what happened in one de-identified episode.
- **Reusable learning:** a candidate pattern, technique consideration or principle that may apply across cases.
- **Published evidence:** a claim extracted from a publication or guideline.
- **Reference concept:** a procedure, technique or contextual concept used to organize knowledge.

A paper's finding is not the same as a local surgeon's observation, and neither automatically overrides the other.

## Node types

| Node | Meaning |
|---|---|
| case_episode | De-identified surgical episode boundary |
| observation | Finding or state observed during the episode |
| decision | A choice made by the surgical team |
| action | A manoeuvre, intervention or deliberate non-action |
| complication_event | An adverse or unexpected event |
| outcome | Patient or procedural outcome without direct identifiers |
| reflection | Retrospective surgeon interpretation or learning |
| evidence_claim | Claim extracted from a publication or guideline |
| procedure / technique | Named procedure or technical approach |
| context_factor | Relevant anatomy, disease severity, prior treatment or operative context |
| general_principle | Reusable learning that has passed a human review gate |

Node payloads are type-specific structured objects. The graph envelope defines shared metadata; it does not prescribe every specialty-specific clinical field.

## Relationship vocabulary

Relationships are directed and typed:

- `occurred_in`: an observation or event belongs to an episode
- `observed_during`: an observation occurred during an episode or phase
- `motivated`: an observation or context factor informed a decision
- `chosen_in_response_to`: a decision addressed a complication or finding
- `performed_during`: an action occurred in an episode
- `responded_to`: an action addressed an event
- `resulted_in` / `contributed_to`: an action or event was followed by an outcome
- `derived_from`: a reflection or candidate learning came from source material
- `supports` / `contradicts`: evidence or experience bears on another claim
- `applies_under` / `not_applicable_under`: scope and context boundaries

Edges have their own provenance and epistemic status. A relationship may be inferred even when both endpoint nodes are directly observed.

## Promotion gate

A case-derived reflection must not become a `general_principle` merely because an LLM generated a plausible summary or because one case had a good outcome.

Promotion requires:
1. Explicit links to source episode(s) and reflections.
2. A clearly stated candidate principle and intended scope.
3. Context and limitations, including known exceptions.
4. Human review recorded in the node's review metadata.
5. Contradictory cases and evidence retained as linked counter-evidence, not deleted.

The schema records review state. Application logic must enforce that only `approved` principles are exposed as approved reusable guidance. JSON Schema alone cannot establish reviewer identity, authorization or clinical validity.

## Example

A difficult laparoscopic cholecystectomy could contain these linked objects:

```text
CaseEpisode
  ├── Observation: acute inflammation
  ├── ComplicationEvent: bleeding
  ├── Decision: change dissection strategy
  ├── Action: rescue manoeuvre
  ├── Outcome: procedure completed
  └── Reflection: what could be improved?
```

Edges express which observation informed the decision, which action responded to the event, and which outcome followed. The sketch is conceptual; persisted relationships are directed edges.

## Portability and compatibility

- The graph has its own format identifier and version 1. It is additive; it does not change the existing `surgical-intelligence-memory` version 1 envelope.
- UUIDs remain stable across export/import.
- Timestamps use RFC 3339 date-times.
- JSON is the interchange contract. Rust structs, SQL tables, vector indexes and model-specific representations are implementation details.
- Future changes require explicit migrations and fixtures. Unknown versions must fail closed.
- Runtime graph validation must reject duplicate IDs, dangling endpoints, invalid temporal intervals and prohibited scope transitions.

## Privacy boundary

Portable fixtures must be synthetic or appropriately de-identified. Do not put names, dates of birth, phone numbers, hospital numbers, addresses, free-text identifiers or re-identification keys in node payloads or provenance. De-identification belongs at the ingestion boundary; graph structure does not make patient data anonymous. Even combinations of rare events may carry re-identification risk and require access controls.

## Implementation status

This document and `schemas/knowledge-graph.schema.json` define the proposed interchange contract. The Rust core, CLI and import/export path do not yet support graph persistence. The next implementation step is to add Rust node/edge types, graph integrity validation, round-trip tests and synthetic fixtures before connecting any registry or dictation adapter.
