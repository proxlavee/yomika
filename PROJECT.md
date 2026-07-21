# Project: yomika

## Architecture
- A Rust-based CLI or app originally named `koharu`, now rebranded to `yomika`.

## Milestones
| # | Name | Scope | Dependencies | Status |
|---|------|-------|-------------|--------|
| 1 | Rebranding | Replace `koharu` with `yomika` in all source files, docs, and README. Integrate `yomika.png`. | none | PLANNED |
| 2 | Code Search | Investigate open issues/PRs from `https://github.com/mayocream/koharu`. Select/prioritize what to port. | none | PLANNED |
| 3 | Port & Test Fixes | Apply chosen upstream fixes/features to the `yomika` codebase. Ensure passing unit/integration tests for each. | M2 | PLANNED |

## Interface Contracts
- CLI and Web/API inputs/outputs must match new branding.

## Code Layout
- Standard Cargo project structure.
- `.agents/skills/` contains Rust skills to be used.
