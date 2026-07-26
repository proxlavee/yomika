# Repository Guidelines

## Project Structure & Module Organization

This Rust workspace pairs a Next.js/Tauri UI. Rust packages live in `crates/`; workspace integration tests are in `tests/integration-tests/`, and crate tests use `crates/*/tests/`. Frontend routes, components, state/adapters, and Vitest suites live under `ui/app/`, `ui/components/`, `ui/lib/`, and `ui/tests/`. Localized documentation is in `docs/`; developer helpers belong in `scripts/`.

## Build, Test, and Development Commands

- `bun install --frozen-lockfile` installs the pinned JavaScript toolchain.
- `bun run dev` launches the Tauri desktop app and bundled UI.
- `bun run build` creates an unbundled release build; platform-native Tauri and GPU prerequisites are required.
- `bun cargo fmt -- --check`, `bun cargo check`, and `bun cargo clippy -- -D warnings` reproduce the Rust CI gates.
- `bun cargo test --workspace --tests` runs Rust unit and integration suites.
- `bun run lint:ui`, `bun run test:ui`, and `bun run format:check` validate frontend code.

## Coding Style & Naming Conventions

Follow `.editorconfig`: four spaces for Rust and two for TypeScript, JSON, CSS, Markdown, and YAML. Use rustfmt, Oxfmt, and Oxlint. Use `snake_case` for Rust functions/modules, `PascalCase` for types/components, `SCREAMING_SNAKE_CASE` for constants, and descriptive TypeScript `camelCase`. Propagate recoverable errors with `?`; avoid production `unwrap()` and undocumented `unsafe`.

## Testing Guidelines

Name UI tests `*.test.ts(x)` beneath `ui/tests/`. Put public Rust behavior in integration tests and focused checks beside modules with `#[cfg(test)]`. Every bug fix needs a regression test. Document hardware-specific ML checks that could not run.

## Commit & Pull Request Guidelines

Use concise Conventional Commit subjects (`fix:`, `feat:`, `test:`, `docs:`, `refactor:`). Pull requests must explain changes and user-visible effects, link issues, list checks, include UI screenshots when relevant, and disclose AI help per `CONTRIBUTING.md`. Exclude credentials, model artifacts, and unrelated changes.

## Agent Working Rules

Respond in English unless the user requests another language. Never assume: inspect files, verify behavior, research unstable claims with primary sources, and run checks before reporting completion. Before acting, evaluate intent, evidence, risks, gaps, and the simplest complete approach. During implementation, reassess and iteratively fix verified relevant gaps or improve code and features without bloat. Never commit, tag, push, package, or publish a release without explicit user authorization. After every code or documentation edit, reread the saved output and inspect the resulting diff manually for mistakes, regressions, omissions, and gaps; correct them before final validation.
