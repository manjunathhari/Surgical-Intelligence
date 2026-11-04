# Architecture

## Separation of responsibilities

1. **Adapters** connect external systems. They own authentication, permissions, input parsing, de-identification, and external schemas.
2. **Ingestion boundary** validates candidate assertions before they enter the core.
3. **Core** owns the domain model, deterministic state changes, event history, and retrieval semantics.
4. **Storage** persists the portable state. The first contract is versioned JSON; a database can later index it without replacing it.
5. **Optional model services** may extract assertions, classify retention, or produce query expansions. They cannot directly mutate state.

## Why not train a small neural network first?

A compact learned model may later improve classification or retrieval, but it does not solve the portability problem by itself. Model weights do not explain the source or history of a particular clinical learning. The primary portable asset should be explicit, versioned knowledge plus provenance and evaluation data. A small model can be added as a replaceable component after we have a benchmark proving where it helps.

## Initial boundaries

The core must be deterministic and runnable offline. No direct database coupling to the surgical registry. No patient identifiers in default memory records. No automatic promotion of a hypothesis into an observed fact.
