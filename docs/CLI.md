# Local CLI

The `si` binary is the first operational interface to the memory core. It works with local, inspectable JSON files and requires no hosted model, database, or network connection.

## Build

From the repository root:

```bash
cargo build --release -p si-cli
```

The executable is `target/release/si` (Windows: `target/release/si.exe`).

## Create a store

```bash
si init --store ./data/surgical-memory.json
```

## Add a structured assertion

```bash
si add \\\
  --store ./data/surgical-memory.json \\\
  --subject "procedure:example" \\\
  --predicate "has_consideration" \\\
  --object "example consideration" \\\
  --statement "Synthetic example learning for CLI testing." \\\
  --source-type "surgeon_reflection" \\\
  --source-id "synthetic-note-001" \\\
  --epistemic-status reported \\\
  --retention durable \\\
  --confidence 0.8 \\\
  --tag synthetic \\\
  --tag example
```

## Inspect and validate

```bash
si list --store ./data/surgical-memory.json
si validate --store ./data/surgical-memory.json
```

## Move state between machines

```bash
si export --store ./data/surgical-memory.json --output ./backup/portable-memory.json
si import --input ./backup/portable-memory.json --store ./restored/surgical-memory.json
```

Import validates the envelope and refuses to overwrite an existing target store. Keep exported files in approved secure storage; the format is portable, not encrypted.

## Import a Surgical Registry export\n\nThe registry adapter is a separate offline crate. The source registry must first create a surgeon-authorized, de-identified JSON export using the contract in [Registry export contract](REGISTRY_EXPORT_CONTRACT.md). Then run:\n\n```bash\nsi import-registry --input ./exports/registry-export.json --output ./data/registry-knowledge-graph.json\n```\n\nThe importer validates the source format, authorization flag, saved/sealed case state, and identifiers in the contract. It writes a portable knowledge-graph envelope and refuses to overwrite an existing output. Node and edge IDs are deterministic for the same pseudonymous source records, so the same export does not create new IDs on each import.\n\nThe import command does not connect to Supabase and does not de-identify raw patient data. Do not run it on an export that has not passed the registry-side privacy and authorization workflow.\n\n## Current limitation

The CLI accepts **structured assertions**. It does not yet convert free-form dictation or conversations into assertions. That extraction layer will be an adapter with review and validation, not hidden inside the deterministic core.
