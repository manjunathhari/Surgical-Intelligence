# Decision 003: Surgical Registry adapter boundary

- **Status:** Accepted architecture; pure mapping crate added
- **Date:** 2026-11-04

The Surgical Registry remains its system of record. Surgical Intelligence consumes a versioned, de-identified JSON export through a separate adapter crate. It must not directly connect to the registry Postgres database or receive Supabase service-role credentials.

The mapper preserves case structure, field provenance, My Experience learning slots and amendments, and explicit case tags. It excludes default/derived values as independent observation signals, does not fabricate decision/action nodes, and does not promote case reflections into general principles.

A trusted registry-side exporter must enforce surgeon scope, consent/purpose authorization, identifier scrubbing and pseudonymization.

The existing My Experience seal captures a historical learning baseline, but learning amendments and outcomes may arrive later. Production ingestion therefore needs append-only source revisions/events and idempotent source IDs rather than treating the first export as immutable.
