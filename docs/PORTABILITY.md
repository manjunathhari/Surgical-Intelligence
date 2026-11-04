# Portability contract

The first interchange format is an inspectable JSON envelope:

- `format`: stable format identifier.
- `version`: integer schema version.
- `exported_at`: RFC 3339 timestamp.
- `state`: assertions and event history.

## Compatibility rules

1. Never silently interpret an unknown major format version.
2. Add optional fields before requiring them.
3. Provide explicit migrations when a version changes.
4. Preserve assertion IDs and provenance during export/import.
5. Test round trips between platforms and process versions.
6. Treat imported files as untrusted input; validate before use.

The long-term goal is portable state across operating systems, databases, and model providers. JSON is the initial interchange contract, not a claim that JSON must be the fastest internal storage format.
