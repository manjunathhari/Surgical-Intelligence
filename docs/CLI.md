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

## Current limitation

The CLI accepts **structured assertions**. It does not yet convert free-form dictation or conversations into assertions. That extraction layer will be an adapter with review and validation, not hidden inside the deterministic core.
