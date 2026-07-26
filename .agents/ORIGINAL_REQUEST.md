# Original User Request

## 2026-07-19T07:42:22Z

<USER_REQUEST>
# Teamwork Project Prompt — Draft

> Status: Launched
> Goal: Craft prompt → get user approval → delegate to teamwork_preview

Rebrand the `the source project` project into a new repository called `yomika`, updating all references and logos. Investigate, verify, and port fixes from the original `the source project`'s issues and pull requests to improve the new `yomika` version.

Working directory: /mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika
Integrity mode: benchmark

## Requirements

### R1. Rebranding
Rename all references from `the source project` to `yomika` across the codebase, including `README.md` and source code. Integrate the `yomika.png` logo.

### R2. Issue and PR Porting
Review the open issues and pull requests from `the upstream source repository`. The agent team should independently decide what to prioritize and fix. Apply relevant fixes, improvements, or features to the `yomika` codebase.

### R3. Test Implementation
For each issue fixed or PR ported, write unit/integration tests to programmatically verify the correctness of the fix or feature.

### R4. Rust Skills Integration
The agent team MUST use the `actionbook/rust-skills` installed in the `.agents/skills/` directory to aid in their tasks.

## Acceptance Criteria

### Codebase Rebranding
- [ ] Searching the codebase for `the source project` (case-insensitive) yields zero results in relevant source and doc files.
- [ ] The `README.md` and UI reflect the new project name `yomika`.

### Upstream Fixes
- [ ] The agent team independently selected, prioritized, and applied fixes/features from the upstream repository.
- [ ] Every implemented fix or feature includes a corresponding passing unit or integration test that programmatically verifies its behavior.
- [ ] The team explicitly utilized the `actionbook/rust-skills` during the execution.
</USER_REQUEST>

## 2026-07-19T07:44:31Z

CRITICAL UPDATE from the user: Do NOT clone the original `the source project` repository to start the project. The base repository has already been created. You MUST clone from our repo instead: `https://github.com/proxlavee/yomika`. Please operate within this cloned yomika repository for all your work, and then port issues/PRs from the source project into it.

## 2026-07-19T07:51:19Z

# Teamwork Project Prompt — Draft

> Status: Launched
> Goal: Craft prompt → get user approval → delegate to teamwork_preview

Rebrand the `the source project` project into a new repository called `yomika`, updating all references and logos. Investigate, verify, and port fixes from the original `the source project`'s issues and pull requests to improve the new `yomika` version.

Working directory: /mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika
Integrity mode: benchmark

## Requirements

### R1. Rebranding
Rename all references from `the source project` to `yomika` across the codebase, including `README.md` and source code. Integrate the `yomika.png` logo.

### R2. Issue and PR Porting
Review the open issues and pull requests from `the upstream source repository`. The agent team should independently decide what to prioritize and fix. Apply relevant fixes, improvements, or features to the `yomika` codebase.

### R3. Test Implementation
For each issue fixed or PR ported, write unit/integration tests to programmatically verify the correctness of the fix or feature.

### R4. Rust Skills Integration
The agent team MUST use the `actionbook/rust-skills` installed in the `.agents/skills/` directory to aid in their tasks.

### R5. Command Safety Constraints
The agents MUST NOT run destructive shell commands such as `rm`, `cp`, or `mv`. All file edits or creations should be done using dedicated file editing tools (`replace_file_content`, `write_to_file`, etc.). Terminal commands should be strictly limited to safe operations like `cargo test`, `cargo build`, or `git` commands.

## Acceptance Criteria

### Codebase Rebranding
- [ ] Searching the codebase for `the source project` (case-insensitive) yields zero results in relevant source and doc files.
- [ ] The `README.md` and UI reflect the new project name `yomika`.

### Upstream Fixes
- [ ] The agent team independently selected, prioritized, and applied fixes/features from the upstream repository.
- [ ] Every implemented fix or feature includes a corresponding passing unit or integration test that programmatically verifies its behavior.
- [ ] The team explicitly utilized the `actionbook/rust-skills` during the execution.

## 2026-07-19T07:55:37Z

CRITICAL UPDATE: The repository has already been cloned and set up in `/mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika`. Do NOT clone `the source project`! Do NOT run `git clone`. You are already inside the `yomika` repo which contains all the necessary files. Please proceed immediately to rebranding the files that are already present in this directory and porting the issues.
