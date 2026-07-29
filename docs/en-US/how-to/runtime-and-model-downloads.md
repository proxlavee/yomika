---
title: Runtime and Model Downloads
---

# Runtime and Model Downloads

Yomika is local-first, but first use is not fully offline. Before the local pipeline can run, Yomika may need to download native runtime files and model weights.

If those downloads fail, check the network path first. Yomika cannot download files from hosts that your machine, ISP, firewall, proxy, or region cannot reach.

## What Yomika downloads

Yomika downloads three broad kinds of assets:

- native runtime packages such as `llama.cpp` binaries, CUDA support files on supported NVIDIA systems, and ZLUDA files on supported Windows AMD systems
- bootstrap model packages needed by the default local page pipeline
- on-demand model packages such as optional OCR or inpainting engines and local GGUF translation models you select later

The important behavior detail is:

- `yomika --download` prepares the bootstrap runtime and model packages, then exits
- it does not download every optional engine or every local LLM shown in the picker

## Where the files go

Yomika stores runtime packages under the configured `Data Path`. The model
library defaults to `<Data Path>/models`, but **Settings > Runtime** can point
it to another absolute folder.

Typical examples:

- Windows: `%LOCALAPPDATA%\Yomika`
- macOS: `~/Library/Application Support/Yomika`
- Linux: `~/.local/share/Yomika`

In the current implementation, the important subdirectories are:

```text
<Data Path>/
  config.toml
  runtime/
    .downloads/
    cuda/
    llama.cpp/
    zluda/
  models/                    # default model-library location
    huggingface/
```

Practical meaning:

- `runtime/.downloads` is the generic archive cache for native runtime downloads
- `runtime/*` contains the extracted libraries Yomika actually loads
- `<Model Library>/huggingface` is the Hugging Face cache used for vision models and local GGUF model files

Not every directory exists on every platform. For example, `zluda/` is Windows-only and only matters on supported AMD setups.

When changing the model library, choose **Use existing** to adopt files already
in the selected folder or **Move current models** to copy Yomika's managed
cache there and remove the old copy after validation. The app restarts after a
successful change so every runtime component uses the same location.

For the configured paths and HTTP settings, see [Settings Reference](../reference/settings.md).

## How runtime downloading works

When Yomika starts, or when you run `yomika --download`, it prepares bootstrap packages for the current platform and compute policy.

At a high level:

1. Yomika creates the runtime and model directories under the current data path.
2. It checks whether each bootstrap package is already current.
3. If a native runtime package is missing or outdated, Yomika downloads the archive into `runtime/.downloads`.
4. It extracts the required files into a runtime-specific install directory and writes an install marker.
5. It preloads the runtime libraries before the local pipeline starts.

In the current source tree, native runtime downloads come from a few different places depending on the package:

- GitHub releases for `llama.cpp`
- PyPI metadata and wheel files for CUDA runtime pieces on supported platforms
- upstream release assets for ZLUDA on supported Windows AMD systems

So a runtime download failure is not always a Hugging Face problem.

## How model downloading works

Most model downloads use the shared Hugging Face cache under
`<Model Library>/huggingface`.

At a high level:

1. Yomika asks for a specific `repo/file` pair.
2. It checks the local Hugging Face cache first.
3. If the file is already cached, Yomika reuses it immediately.
4. If not, Yomika downloads that exact file and stores it in the Hugging Face cache layout.
5. Later loads reuse the cached file instead of redownloading it.

This is true for the default vision stack and for local GGUF translation models that Yomika downloads on demand.

## Manage downloads and disk space

Local translation models have separate **Download** and **Load** actions. A
missing model is never downloaded by **Load**. While a download is active, use
**Cancel** in the model picker or notification area; Yomika removes partial
model files and reports the final state.

Use **Settings > Runtime** to:

- view model-library and temporary-download usage
- clear completed runtime archives and partial model files
- delete one downloaded local translation model
- delete all downloaded model files, including the default vision stack
- delete and immediately download a local translation model again

Unload a local translation model and finish or cancel active jobs/downloads
before changing its files. Deleted default models are downloaded again the
next time their pipeline stage needs them.

## What Hugging Face is

[Hugging Face](https://huggingface.co/) is a model hosting platform. In Yomika, it is mostly the place where many model files live.

Hugging Face is not the thing doing the inference for Yomika. Yomika downloads model files from Hugging Face, caches them locally, and then runs them on your machine through the local runtime stack.

If Hugging Face is blocked on your network, Yomika cannot fetch those model files no matter which button you click in the app.

## What "internet connection" means here

For Yomika, "I have internet" means more than "my Wi-Fi icon is connected" or "Google opens in a browser".

What actually matters is:

- DNS can resolve the required host names
- HTTPS connections can be established
- file downloads can start and finish
- your ISP, firewall, proxy, antivirus, or region is not blocking the host

Being able to open unrelated sites does not prove that `huggingface.co`, `github.com`, or `pypi.org` is reachable from your current network.

## Test the connection outside Yomika first

If these checks fail outside Yomika, fix the network path first. That is not a Yomika bug.

### Browser checks

Open these in a normal browser:

- `https://huggingface.co`
- `https://huggingface.co/ogkalu/comic-text-and-bubble-detector`
- `https://github.com`
- `https://pypi.org`

If they do not load in a browser, Yomika will not be able to load them either.

### macOS and Linux checks

```bash
nslookup huggingface.co
curl -I --max-time 20 https://huggingface.co
curl -I --max-time 20 https://github.com
curl -I --max-time 20 https://pypi.org
curl -L --max-time 20 -o /dev/null -w '%{http_code}\n' \
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

What you want to see:

- `nslookup` returns an address instead of a DNS failure
- the `curl -I` commands return a normal HTTPS response such as `200`
- the direct file test prints `200`

### Windows PowerShell checks

Use `curl.exe`, not PowerShell's `curl` alias:

```powershell
nslookup huggingface.co
curl.exe -I --max-time 20 https://huggingface.co
curl.exe -I --max-time 20 https://github.com
curl.exe -I --max-time 20 https://pypi.org
curl.exe -L --max-time 20 -o NUL -w "%{http_code}\n" `
  https://huggingface.co/ogkalu/comic-text-and-bubble-detector/resolve/main/config.json
```

The same expectations apply: normal DNS resolution, successful HTTPS responses, and `200` for the direct file test.

## How to tell if Hugging Face is blocked in your area or on your network

These are the usual signs:

- `huggingface.co` times out, resets, or never finishes loading while unrelated sites still work
- the direct file test above never returns `200`
- the same failure happens in a browser, in `curl`, and in Yomika
- the command works on another network such as a mobile hotspot but fails on your normal network
- GitHub or PyPI works, but Hugging Face does not

If changing networks fixes it, the network path is the problem.

If Hugging Face fails everywhere outside Yomika on the same machine, solve that first before filing a Yomika bug.

## Test with Yomika after the external checks pass

Once the browser and `curl` checks work, test Yomika directly:

```bash
# macOS / Linux
yomika --download --debug
yomika --cpu --download --debug

# Windows
yomika.exe --download --debug
yomika.exe --cpu --download --debug
```

Why both commands help:

- `--download` tests the normal bootstrap download path
- `--cpu --download` skips GPU preference so you can separate network problems from GPU-runtime preparation problems

If one of those commands fails, keep the exact error text. It is much more useful than "download broken".

## Before filing a bug

Check these first:

- can a browser open Hugging Face, GitHub, and PyPI from the same machine
- do the `curl` tests above succeed outside Yomika
- does the problem change on another network
- does `--cpu --download` behave differently from `--download`
- what is your configured `Data Path`
- what exact error text did Yomika print

If the host is unreachable outside Yomika, open a network, firewall, proxy, VPN, or ISP issue first.

If the external checks pass and Yomika still fails, then file a Yomika bug and include the details above.
