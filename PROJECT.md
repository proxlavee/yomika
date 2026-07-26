# Project: Yomika

## Architecture

Yomika is a Rust workspace with a Tauri desktop shell and a statically exported Next.js editor. The Rust backend owns project state, history, model execution, rendering, HTTP APIs, and MCP tools; React Query treats that backend as the source of truth.

## Current Priorities

| Area | Scope | Status |
| --- | --- | --- |
| Identity | Yomika names, assets, crates, executable, docs, and storage formats | In progress |
| Reliability | Confirmed editor, runtime-loader, and geometry regressions | Implemented |
| Memory | Bounded GPU FFT plans and short-lived browser image blobs | Implemented |
| Verification | Formatting, unit tests, lint, Cargo metadata, and build checks | Pending |

## Interface Contracts

- Managed project directories end in `.ymkproj`; portable archives end in `.ymk`.
- The local API remains under `/api/v1`; the MCP endpoint is `/mcp`.
- Pipeline engine identifiers use the `yomika-*` namespace.
- Hosted providers are opt-in and must not receive page data unless selected by the user.
