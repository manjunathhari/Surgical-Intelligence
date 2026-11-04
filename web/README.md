# Local Workbench

## Requirements
- Rust stable toolchain
- Node.js 20+

## Run locally
From the repository root, use two terminals.

Terminal 1 — Rust API:
```bash
cargo run -p si-api
```
The API binds only to `127.0.0.1:8787`. By default it creates/uses `./data/state-v2.json`. To use another portable state file, set `SI_STATE_PATH` before starting.

Terminal 2 — React UI:
```bash
cd web
npm install
npm run dev
```
Open http://localhost:5173.

The browser reads live state and summary from the Rust API. The State Inspector can submit edited JSON, but the API validates it with `si-core` before replacing the state file. Back up state before editing. This initial workbench is local development software, not a clinical decision-support interface.
