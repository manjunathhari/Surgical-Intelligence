# Safety, privacy, and clinical boundaries

Surgical Intelligence is an engineering project for structured knowledge management and research support. The initial core is not a medical device and must not be represented as a validated clinical decision-support system.

## Default data policy

- Do not commit patient records, identifiable clinical notes, recordings, images, or exports.
- Use synthetic fixtures for development and public tests.
- De-identification must happen at a controlled ingestion boundary; removing names alone is not sufficient.
- Treat dates, rare procedures, locations, and unusual clinical narratives as potentially identifying in combination.
- Store the minimum information required for the stated learning objective.
- Keep source permissions and access control in the adapter / application layer.
- Imported memory files are untrusted input and must be validated before use.

## Knowledge quality

- Retain provenance for every assertion.
- Keep observed, reported, inferred, and hypothesized information distinct.
- Do not convert confidence scores into probabilities unless calibrated against an appropriate dataset.
- Preserve disputes, retractions, and supersession history.
- Do not collapse patient-specific outcomes into generalizable surgical recommendations without an explicit review and evidence process.

## Human oversight

Any future clinical-facing output must clearly communicate source, uncertainty, applicability, and limitations. Clinical decisions remain with qualified clinicians using the full patient context and applicable standards of care.
