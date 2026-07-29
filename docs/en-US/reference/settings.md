---
title: Settings Reference
---

# Settings Reference

Yomika's Settings screen currently exposes seven main areas:

- `Appearance`
- `Engines`
- `API Keys`
- `AI`
- `Keybinds`
- `Runtime`
- `About`

This page documents the current settings surface as implemented in the app.

## Appearance

The `Appearance` tab currently includes:

- theme: `Light`, `Dark`, or `System`
- UI language from the bundled translation list
- `Rendering Font`, which is used when Yomika renders translated text onto the page

Theme, language, and rendering-font changes apply immediately in the frontend.

## Engines

The `Engines` tab selects the backend used for each pipeline stage:

- `Detector`
- `Bubble Detector`
- `Font Detector`
- `Segmenter`
- `OCR`
- `Translator`
- `Inpainter`
- `Renderer`

These values are stored in the shared app config and save immediately when changed.

## API Keys

The `API Keys` tab currently covers these built-in providers:

- `OpenAI`
- `Gemini`
- `Claude`
- `DeepSeek`
- `DeepL`
- `Google Cloud Translation`
- `Caiyun`
- `OpenAI Compatible`

Each provider appears as an accordion with a status dot:

- green — ready (key saved and discovery succeeded)
- amber — missing required configuration (API key or, for `OpenAI Compatible`, a base URL)
- red — discovery failed against the configured endpoint
- grey — no configuration yet

Current behavior:

- provider API keys are not written to `config.toml`
- on macOS and Windows, provider API keys are stored through the system keyring
- on Linux, provider API keys are stored in Yomika's local filesystem credential store under the app data directory with owner-only file permissions
- provider base URLs are stored in the app config
- `OpenAI Compatible` requires a custom `Base URL`; models are discovered dynamically by calling `GET /v1/models` against that URL
- machine-translation providers (`DeepL`, `Google Cloud Translation`, `Caiyun`) only need an API key; `Caiyun` supports a limited set of target languages
- clearing a key removes it from credential storage

The API response intentionally redacts saved keys rather than returning the raw secret.

The Linux filesystem credential store relies on local filesystem permissions rather than OS-level encryption.

## AI

The `AI` tab manages the optional Codex connection used by the image-generation
workflow. It shows the current account state and provides device-code sign-in
and sign-out actions. This is separate from the provider keys configured in
`API Keys`.

## Keybinds

The `Keybinds` tab lets you rebind tool-switch and brush-size shortcuts plus the undo and redo bindings.

Current behavior:

- defaults are `V`/`M`/`B`/`E`/`R` for the Select / Block / Brush / Eraser / Repair Brush tools
- defaults are `[` and `]` for the brush size step
- defaults are `Ctrl + Z` and `Ctrl + Shift + Z` (`Cmd + Z` and `Cmd + Shift + Z` on macOS) for undo and redo
- mouse-wheel zoom, Hand tool or `Ctrl` + drag panning, select-all (`Ctrl + A`), and the legacy `Ctrl + Y` redo fallback are not rebindable
- conflicts are highlighted in the editor; you can reset to defaults from the same screen

Keybind preferences are stored in the frontend preferences layer, not in `config.toml`.

For the full default list, see [Keyboard Shortcuts](keyboard-shortcuts.md).

## Runtime

The `Runtime` tab groups shared runtime configuration and model-storage
maintenance:

- `Data Path`
- `Model Library`
- `HTTP Connect Timeout`
- `HTTP Read Timeout`
- `HTTP Max Retries`

Current behavior:

- `Data Path` controls where Yomika stores runtime packages, page manifests, and image blobs
- `Model Library` defaults to `<Data Path>/models`, or can use another absolute folder
- **Use existing** adopts a model cache already in the selected folder; **Move current models** validates and moves Yomika's managed cache there
- the storage panel reports model and temporary-download usage, clears temporary files, and deletes or redownloads local models
- changing the model-library path requires a restart; unload local models and finish or cancel active work first
- `HTTP Connect Timeout` sets how long Yomika waits while establishing HTTP connections
- `HTTP Read Timeout` sets how long Yomika waits while reading HTTP responses
- `HTTP Max Retries` controls automatic retries for transient HTTP failures
- these HTTP values are used by the shared runtime HTTP client for downloads and provider-backed requests
- applying changes saves the config and restarts the desktop app because the runtime client is built at startup

## About

The `About` tab currently shows:

- the current app version
- whether a newer GitHub release exists
- the `proxlavee` author link
- the repository link

The version check compares the local app version against the latest GitHub
release for `proxlavee/yomika`. It runs at startup and manually from About. An
available update opens the Releases page; Yomika does not download or install
application updates.

## Persistence model

The current settings behavior is split across storage layers:

- `config.toml` stores shared app config such as data and model paths, `http`, `pipeline`, and provider `baseUrl`
- provider API keys are stored separately from `config.toml` through the platform credential store described above
- theme, language, and rendering-font preferences are stored in the frontend preferences layer

That means clearing frontend preferences is not the same as clearing saved provider API keys or shared runtime config.

## Related pages

- [Use OpenAI-Compatible APIs](../how-to/use-openai-compatible-api.md)
- [Models and Providers](../explanation/models-and-providers.md)
- [HTTP API Reference](http-api.md)
