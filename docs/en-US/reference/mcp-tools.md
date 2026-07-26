---
title: MCP Tools Reference
---

# MCP Tools Reference

Yomika exposes MCP tools at:

```text
http://127.0.0.1:<PORT>/mcp
```

The MCP server uses the streamable HTTP transport from `rmcp 1.5` and operates on the same project, scene, and pipeline state as the GUI and HTTP API.

## What the MCP server exposes today

The current implementation deliberately exposes a small, low-level surface centred on the project lifecycle, the history layer, and pipeline jobs. Fine-grained edits go through `yomika.apply` with an `Op` payload rather than dedicated per-field tools.

If you need richer inspection (page thumbnails, image layers, font lists, scene snapshots), use the [HTTP API](http-api.md) directly. The two run side-by-side on the same port and share a single in-process state.

## Tools

| Tool                    | Purpose                                                  | Parameters                                                                       |
| ----------------------- | -------------------------------------------------------- | -------------------------------------------------------------------------------- |
| `yomika.apply`          | apply an `Op` to the active scene                        | `op` — JSON-tagged `Op` value                                                    |
| `yomika.undo`           | revert the most recent op                                | none                                                                             |
| `yomika.redo`           | re-apply the most recent undone op                       | none                                                                             |
| `yomika.open_project`   | open or create a Yomika project directory                | `path`, optional `createName`                                                    |
| `yomika.close_project`  | close the active project                                 | none                                                                             |
| `yomika.start_pipeline` | start a pipeline run; returns a `jobId`                  | `steps[]`, optional `pages[]`, `targetLanguage`, `systemPrompt`, `defaultFont`   |

### `yomika.apply`

Applies a single mutation to the scene through the history layer. The `op` value is the same JSON-tagged `Op` enum the HTTP API accepts at `POST /history/apply` — common variants include `AddPage`, `RemovePage`, `AddNode`, `UpdateNode`, `RemoveNode`, and `Batch`.

Returns `{ epoch }` — the new scene epoch after the op is applied.

### `yomika.undo` / `yomika.redo`

Walk the history stack one step in either direction. Both return `{ epoch }` where `epoch` is `null` at a stack boundary (nothing left to undo or redo).

### `yomika.open_project`

Opens an existing project directory or creates one at the supplied path. Pass `createName` to create a new project under the path; omit it to open whatever is already there.

Returns `{ name, path }` for the now-active session.

### `yomika.close_project`

Closes the current session. Subsequent calls that require a project return an `invalid request` error until another project is opened.

### `yomika.start_pipeline`

Spawns a pipeline run in the background. `steps` is an ordered list of engine ids registered through the pipeline `Registry` (validated against `GET /api/v1/engines`). Omit `pages` to run on every page in the project; pass a list of `PageId`s to scope the run to a subset.

Returns `{ jobId }` immediately. Progress and completion are published on the HTTP `/events` stream as `JobStarted`, `JobProgress`, `JobWarning`, and `JobFinished`. The MCP transport itself does not stream job progress — you watch SSE for that.

## Suggested agent flow

Most agent sessions look like this:

1. `yomika.open_project` — point at a managed project directory
2. read `GET /api/v1/scene.json` over HTTP to inspect the scene
3. either:
    - apply scoped edits via `yomika.apply` with explicit `Op` payloads, or
    - run an end-to-end pipeline via `yomika.start_pipeline` and watch `GET /api/v1/events`
4. export through `POST /api/v1/projects/current/export` over HTTP
5. `yomika.close_project`

`yomika.undo` and `yomika.redo` are useful when an op turns out to be wrong and you want to back out instead of computing the inverse manually.

## Related pages

- [Configure MCP Clients](../how-to/configure-mcp-clients.md)
- [Run GUI, Headless, and MCP Modes](../how-to/run-gui-headless-and-mcp.md)
- [HTTP API Reference](http-api.md)
