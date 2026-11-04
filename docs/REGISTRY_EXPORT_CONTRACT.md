# Surgical Registry export contract

**Format:** `surgical-registry-export`
**Version:** `1`
**Schema:** [`schemas/registry-export.schema.json`](../schemas/registry-export.schema.json)

This is the explicit boundary between `surgical-registry` and `Surgical-Intelligence`. The registry remains the system of record. The importer accepts only a server-generated, surgeon-authorized, de-identified export; it does not connect to the registry database.

## Minimum envelope

```json
{
  "format": "surgical-registry-export",
  "version": 1,
  "exported_at": "2026-11-04T00:00:00Z",
  "source": {
    "system": "surgical-registry",
    "schema_family": "postgres-case-values-learning-v1",
    "registry_version": null
  },
  "cases": []
}
```

Every case must contain:
- `source_case_ref`: keyed pseudonym for the registry case, never its UUID or study ID.
- `surgeon_ref`: optional keyed pseudonym for the surgeon of record.
- `export_authorized: true`: set only after a distinct, documented purpose/consent authorization check.
- `procedure_module`, `procedure_label`, `surgery_date`, and saved `status`.
- `values`: current `case_values` with the latest `field_provenance` source, evidence and timestamp.
- `field_history`: all `field_provenance` records in chronological order, with actor pseudonyms only.
- `learning_sealed_at`, `learning_answer`, and `learnings`.
- Each learning's original slot text and origin, sign tags, and append-only amendments.
- Categorized case `tags`.

## Source mapping rules

1. Preserve the current value and all historical values. Do not replace field history with only the latest value.
2. Registry source labels map as follows: `said`, `chip`, `manual`, and `histopath_photo` are reported; `default` and `derived` are inferred; unknown source values remain untrusted hypotheses.
3. All values remain available in the graph. The adapter marks default, derived and unknown-origin values as ineligible to independently create learning signals.
4. The My Experience slots are authoritative surgeon-authored records. Keep the original slot, exact text and origin. `saw` and `first_told` are observations; `did` is an action. The `options` slot describes alternatives, not the decision selected.
5. Outcome mapping is allowed only for a reviewed allow-list of explicit field names. Unknown fields remain neutral observations.
6. Learning amendments are later events. Never fold them into the baseline or delete the previous words.
7. A learning seal means "baseline captured", not "episode complete".

## Authorization and privacy requirements

The exporter must enforce a distinct authorization decision for the Surgical Intelligence purpose. The registry's research-consent field must not be assumed to authorize another processing purpose.

Before export:
- De-identify all free text and evidence fields using the registry's fail-closed privacy pipeline and a second export-boundary check.
- Exclude patient names, UHID/study ID, registry UUIDs, dates of birth, phone numbers, addresses, key-sheet identifiers, hospital/site IDs, user IDs, email, initials and raw media references.
- Exclude raw transcripts, audio, photos, clips, storage paths and re-identification keys.
- Generate HMAC pseudonyms for source case, surgeon and actor identities. Keep the key only within the source registry's secure environment.
- Shift/coarsen surgery dates, learning timestamps and field history consistently per surgeon to reduce date-based linkage while preserving temporal intervals.
- Retain an export manifest with schema version and a source revision watermark.

Pseudonymized episodes remain sensitive and must receive access controls, audit logging and a retention policy. The importer cannot verify whether a source registry has actually fulfilled these obligations; it only validates the envelope and the exporter's authorization attestation.

## Known implementation boundary

The Rust adapter and CLI importer are implemented in Surgical Intelligence. A registry-side route/job that produces this envelope is **not** implemented and the source registry repository has not been modified. Until that exporter exists and is reviewed, create fixture exports only; do not point the adapter at live patient data.
