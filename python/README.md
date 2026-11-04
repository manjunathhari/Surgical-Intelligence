# Python evaluation harness

This package is intended for offline evaluation of ingestion, retrieval, memory updates, and portability.

## Setup

From the repository root:

```bash
python -m venv .venv
# Windows PowerShell: .venv\\Scripts\\Activate.ps1
# macOS/Linux: source .venv/bin/activate
python -m pip install -e "python/[dev]"
python -m pytest python/tests
```

## Initial metrics

- `precision_at_k`
- `recall_at_k`

These are baseline utilities, not a complete benchmark. Future benchmark suites should separately evaluate:
- assertion extraction fidelity;
- provenance preservation;
- correct separation of stable and temporary memories;
- contradiction and update handling;
- retrieval relevance and evidence coverage;
- export/import equivalence across machines and versions;
- latency, memory footprint, and state growth.
