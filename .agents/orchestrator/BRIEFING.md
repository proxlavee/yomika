# BRIEFING — 2026-07-19T10:55:00+03:00

## Mission
Manage the rebranding of the source project to yomika, and port fixes/features from the upstream source repository to the proxlavee/yomika repository, while strictly adhering to safety constraints (no rm/cp/mv) and testing requirements.

## 🔒 My Identity
- Archetype: teamwork_preview_orchestrator
- Roles: orchestrator, user_liaison, human_reporter, successor
- Working directory: /mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika/.agents/orchestrator
- Original parent: top-level
- Original parent conversation ID: 329b734a-ff1a-4fad-93a8-55910216e13b

## 🔒 My Workflow
- **Pattern**: Project
- **Scope document**: /mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika/PROJECT.md
1. **Decompose**: Split work into Milestones: Rebranding, Upstream Fixes Porting (including issue review and prioritization), and E2E Testing.
2. **Dispatch & Execute**: Delegate (sub-orchestrator)
3. **On failure**: Retry, Replace, Skip, Redistribute, Redesign, Escalate.
4. **Succession**: At 16 spawns, write handoff.md, spawn successor.
- **Work items**:
  1. Rebranding (code, README, logo) [pending]
  2. Issue/PR Porting and Tests [pending]
- **Current phase**: 1
- **Current focus**: Dispatching M1 Rebranding sub-orchestrator.

## 🔒 Key Constraints
- NEVER run destructive shell commands (rm, cp, mv). Use file edit tools instead.
- Terminal commands limited to safe operations like `cargo test`, `cargo build`, or `git`.
- ALL features/fixes must include passing unit/integration tests programmatically verifying the behavior.
- MUST use the `actionbook/rust-skills` installed in `.agents/skills/`.
- Never reuse a subagent after it has delivered its handoff — always spawn fresh.

## Current Parent
- Conversation ID: 329b734a-ff1a-4fad-93a8-55910216e13b
- Updated: 2026-07-19T10:55:00+03:00

## Key Decisions Made
- Use Project Orchestration pattern.
- Divide the project into Rebranding Track and Porting Track, plus E2E Testing Track.

## Team Roster
| Agent | Type | Work Item | Status | Conv ID |
|-------|------|-----------|--------|---------|

## Succession Status
- Succession required: no
- Spawn count: 0 / 16
- Pending subagents: none
- Predecessor: none
- Successor: not yet spawned

## Active Timers
- Heartbeat cron: not started
- Safety timer: none

## Artifact Index
- /mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika/.agents/ORIGINAL_REQUEST.md — Original User Request
- /mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika/PROJECT.md — Global architecture and milestones
- /mnt/c/Users/KaanReyiz/Desktop/KodProjeleri/yomika/.agents/orchestrator/progress.md — Progress tracking
