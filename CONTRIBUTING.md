# Contributing to Yomika

Thank you for improving Yomika. Start by reading [AGENTS.md](AGENTS.md), then search [existing issues](https://github.com/proxlavee/yomika/issues) before opening a duplicate.

## Development Workflow

1. Install dependencies with `bun install --frozen-lockfile`.
2. Keep changes focused and add a regression test for behavior fixes.
3. Run Rust formatting and the affected Cargo tests.
4. Run `bun run format:check`, `bun run lint:ui`, and `bun run test:ui` for UI changes.
5. Describe user-visible effects, platform limitations, and checks in the pull request.

Use concise Conventional Commit subjects such as `fix: preserve color picker drag value` or `refactor: rename project archive format`. Do not include generated build output, credentials, downloaded models, or unrelated cleanup.

## Pull Requests

Link relevant issues and explain both the cause and the chosen fix. Include screenshots for visual changes and note any hardware-specific checks that could not run. Keep API, archive-format, and configuration changes explicit because they may affect existing clients or projects.

## AI-Assisted Contributions

Disclose meaningful AI assistance in the pull request. Contributors remain responsible for understanding, reviewing, testing, and licensing every submitted change. Unverified generated output is not a substitute for a reproducible test or source-backed explanation.
