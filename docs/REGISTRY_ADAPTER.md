# Surgical Registry adapter

The workspace includes `si-registry-adapter`, a pure, offline mapper from a de-identified Surgical Registry export to the Surgical Intelligence knowledge-graph envelope. It has no registry credentials and no database/network access.

## Source architecture inspected

The registry is a Next.js 15 / TypeScript application backed by Postgres/Supabase. Its current schema and code separate:
- `cases`: coded case identity, surgeon/site attribution, procedure module, surgery date and lifecycle status.
- `case_values`: generic JSONB current field values across procedure modules.
- `field_provenance`: append-only value source/evidence history (`said`, `default`, `derived`, `chip`, `histopath_photo`, `manual`).
- `transcripts`: de-identified text; audio is not stored.
- `case_learning`, `learnings`, `learning_items`: structured surgeon-owned reflections with labelled slots and origin.
- `learning_amendments`: append-only edits after the learning seal.
- `tags` / `case_tags`: categorized case labels.
- `experience.entries`: internal surgeon-only semantic index of learning text, embeddings and sign tags.

The adapter deliberately reads the **source of meaning**, not the embedding index. My Experience can contain up to three learnings per case. Slots include own words, expected, saw, did, what first told you, options, what it told you, reminded me, confidence, changed mind, fellow and lesson. Each item retains whether it came from own words, a box or a prompted question. The first Save case seals a baseline; later amendments are separate append-only rows. Both the baseline and amendment history must survive export.

## Mapping implemented

| Registry source | Surgical Intelligence representation |
|---|---|
| Saved/sealed case | `case_episode` node |
| Current `case_values` | Typed case-scoped observation/outcome nodes with source and evidence |
| Full `field_provenance` | Time-stamped historical observation nodes; earlier values are marked superseded |
| My Experience record | Parent `reflection` node with every labelled slot, origin and amendment preserved |
| My Experience `saw` / `first_told` slots | Explicit case-scoped `observation` child nodes |
| My Experience `did` slot | Explicit case-scoped `action` child node |
| Case tags | Case-scoped `context_factor` nodes with category preserved |
| Default/derived/unknown-origin fields | Retained, but marked `learning_signal_eligible=false` |
| Surgeon of record | Optional keyed pseudonym; no user ID, name, email or initials |

The mapper does **not** infer a selected decision from the registry's "other options" text box. Nor does it infer a technical manoeuvre from `approach=Robotic`. Such a mapping requires a specific recorded decision/action, not a guess from a generic field.

Outcome classification is limited to an explicit allow-list of registry field keys (e.g. Clavien–Dindo, readmission, death, length of stay, discharge destination and reintervention). Everything else remains a neutral observation until an explicit reviewed mapping is added.

## Import

Build the CLI and import an authorized export:

```bash
cargo build --release -p si-cli
si import-registry --input ./exports/registry-export.json --output ./data/registry-knowledge-graph.json
```

The importer validates the export format/version, source schema family, per-case authorization flag, saved status and learning seal. It emits a separate portable graph envelope. It refuses to overwrite an existing output.

Node and edge IDs are deterministic for a given pseudonymous case, field, timestamp and learning ordinal/slot. Re-importing an unchanged source record therefore produces the same IDs. This is not yet a graph-store merge engine: when a source changes, the receiving store must apply the new records and their lifecycle semantics rather than replacing history indiscriminately.

## Required registry-side exporter (not implemented in this repository)

The source registry still needs a controlled exporter. Its job is to:
1. Run server-side through an authenticated owner/surgeon action or a dedicated, least-privilege service identity.
2. Scope every export to the authorized surgeon/registry. Preserve the surgeon of record and field-change actor only as keyed pseudonyms.
3. Exclude unsaved, cancelled, unsealed or unauthorized cases.
4. Export current `case_values` joined to the latest `field_provenance`, plus the full `field_provenance` history.
5. Export `learnings`, `learning_items` and ordered `learning_amendments`; do not use `experience.entries` as the canonical source.
6. Export categorized `tags` and sign tags without photo/clip references.
7. Run a second de-identification pass on every evidence string and learning slot; fail closed if it cannot run.
8. Exclude registry case UUID, study ID, hospital/site IDs, user IDs, email, initials, key-sheet IDs, transcripts, audio, photos, clips, storage paths and re-identification keys.
9. Enforce a distinct purpose-authorization gate. `consent_research` alone must not be interpreted as permission to export into a separate knowledge system.
10. Generate stable HMAC-based pseudonyms for case, surgeon and actor references. Never export the secret key.
11. Shift or coarsen surgery dates and all related case timestamps with a consistent per-surgeon policy before export. Preserve relative intervals and ordering; exact dates can be identifying.
12. Include an export manifest/cursor (e.g. source revision watermark) so later amendments, field history and follow-up outcomes can be synchronized idempotently.

The JSON export contract is documented separately in [Registry export contract](REGISTRY_EXPORT_CONTRACT.md).

## Important time semantics

The first Save case seals the learning baseline before later outcomes may be known. Learnings can be amended after the seal, and histology, complications and follow-up can arrive later. Therefore, the seal is a historical checkpoint, not an immutable completed episode. Later syncs must append amendments and outcome/follow-up events without rewriting the original reflection.

## Privacy and governance

Pseudonymization is not anonymization. Rare operations, date patterns and event combinations can be re-identifying. Require purpose limitation, access controls and retention policies in Surgical Intelligence. This mapper does not process raw images, video or audio and does not promote any case reflection into a reusable surgical principle.
