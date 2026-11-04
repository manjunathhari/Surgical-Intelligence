# Surgical Registry adapter

This is a pure, offline mapper from a de-identified Surgical Registry export to Surgical Intelligence graph nodes and edges. It has no registry credentials and no database/network access.

## Source architecture inspected

The registry is a Next.js 15 / TypeScript application backed by Postgres/Supabase. The important data structures are:
- `cases`: coded case identity, surgeon/site attribution, procedure module, date and lifecycle status.
- `case_values`: generic JSONB field-value store across modules.
- `field_provenance`: append-only value source/evidence history (said, default, derived, chip, histopath_photo, manual).
- `transcripts`: de-identified text, while audio is not stored.
- `case_learning` + `learnings` + `learning_items`: structured surgeon reflections with labelled slots and origin.
- `learning_amendments`: append-only edits after the learning seal.
- `tags` / `case_tags`: case context labels.
- `experience.entries`: an internal surgeon-only semantic index of learning text, embeddings and sign tags.

The registry's My Experience is intentionally surgeon-owned, not a team-wide note. It has up to three learning records per case with own words, expected, saw, did, what first told you, options, what it told you, reminded me, confidence, changed mind, fellow, lesson; each item records whether it was own words, a box or prompted. After the first Save case, learnings are sealed as a baseline; later amendments are retained separately. This historic state and the amendment stream must both be preserved.

## Mapping

| Registry | Surgical Intelligence |
|---|---|
| Saved/sealed case | `case_episode` |
| Explicit, non-derived `case_values` | case-scoped `observation`, with field/value/source/evidence |
| My Experience learning | case-scoped `reflection`; slot, origin, signs and amendments preserved |
| Case tags | case-scoped `context_factor` |
| Derived/default field values | not treated as independent observation signals |
| Outcome/follow-up after seal | separate later event in subsequent adapter version |

This adapter does not infer decision/action nodes from fields like `approach=Robotic`. Such mappings require an explicit event/decision record, not a guess based on a generic value.

## Required registry-side exporter (not yet implemented here)

1. Run server-side using a dedicated service identity or authenticated surgeon action.
2. Scope every export to the authorized surgeon/registry; preserve actor attribution as pseudonyms only.
3. Exclude unsaved, cancelled, or unsealed cases by default.
4. Join current values to latest `field_provenance`; preserve origin and evidence.
5. Export `learning_items` and ordered `learning_amendments`, not just the flattened embedding text in `experience.entries`.
6. Run a second de-identification pass over every evidence string and learning slot; fail closed.
7. Exclude registry case UUID, study ID, hospital/site IDs, user IDs, email, initials, key-sheet IDs, transcript/audio/photo/clip data, storage paths and re-identification keys.
8. Gate export by explicit consent and purpose authorization. `consent_research` should not be assumed to authorize a separate knowledge-engine use.
9. Use stable keyed pseudonyms and an export cursor/revision manifest for idempotent repeated sync.
10. Ship the versioned JSON to a separate authenticated import endpoint or operator-controlled encrypted file.

## Important time semantics

The registry's first Save case seals learning *before* later outcomes may be known. A learning can receive post-seal amendments, and complications/histology/follow-up can be captured later. So the adapter must not treat the initial seal as an immutable complete episode. Future sync should append new amendments and outcome/follow-up events separately, retaining the earlier state.

## Privacy and governance

Pseudonymization is not anonymization. Rare operations, dates and event combinations can be re-identifying. Require access controls and retention policies on the Surgical Intelligence side too. This initial mapper does not process raw images, video or audio and does not promote any reflection into a reusable surgical principle.
