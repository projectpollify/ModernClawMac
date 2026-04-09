# RUNBOOK

## Purpose

This runbook records the bring-up, recovery, and basic verification steps for the ModernClaw base workspace.

## Current Workspace

- Repo root: `C:\Users\pento\Desktop\ModernClawBase`
- App source: `C:\Users\pento\Desktop\ModernClawBase\local-ai`

## Daily Bring-Up

1. Make sure Ollama is installed.
2. Start Ollama if it is not already running.
3. Start the app in Tauri dev mode.

### Commands

```powershell
cd "C:\Users\pento\Desktop\ModernClawBase\local-ai"
npm run tauri:dev
```

If Ollama is not already running, start it in a separate PowerShell window:

```powershell
ollama serve
```

## Build Verification

### Frontend + TypeScript

```powershell
cd "C:\Users\pento\Desktop\ModernClawBase\local-ai"
npm run build
```

### Rust / Tauri Backend

```powershell
cd "C:\Users\pento\Desktop\ModernClawBase\local-ai\src-tauri"
cargo check
```

## Packaged Build

```powershell
cd "C:\Users\pento\Desktop\ModernClawBase\local-ai"
npm run tauri:build
```

## Common Recovery Steps

### App folder was renamed or moved

If Tauri build paths or generated artifacts look stale:

```powershell
cd "C:\Users\pento\Desktop\ModernClawBase\local-ai\src-tauri"
cargo clean
cd "C:\Users\pento\Desktop\ModernClawBase\local-ai"
npm run tauri:dev
```

### Sidebar appears lost

If the app gets into a strange UI state, restart dev mode.

## Voice Setup Notes

### Current Voice Features

Working now:

- local voice output through Piper
- local voice input through Whisper
- pause, resume, and stop playback controls
- shared machine-level tool paths with workspace-level voice preferences

### Current Default Voice Tool Layout

The app provisions shared folders under the LocalAI app-data root:

- `%APPDATA%\LocalAI\tools\piper\`
- `%APPDATA%\LocalAI\tools\piper\voices\`
- `%APPDATA%\LocalAI\tools\whisper\`
- `%APPDATA%\LocalAI\tools\whisper\models\`

Important current limitation:

- the folders are auto-created
- Piper and Whisper executables and model files are not auto-downloaded yet
- clean-machine setup still requires manual dependency placement or installation

### Current Validated Piper Setup

- Piper executable: `C:\Tools\piper\piper.exe`
- Amy model: `%APPDATA%\LocalAI\tools\piper\voices\en_US-amy-medium.onnx`
- Amy metadata: `%APPDATA%\LocalAI\tools\piper\voices\en_US-amy-medium.onnx.json`
- Joe model: `%APPDATA%\LocalAI\tools\piper\voices\en_US-joe-medium.onnx`
- Joe metadata: `%APPDATA%\LocalAI\tools\piper\voices\en_US-joe-medium.onnx.json`

### Current Validated Whisper Setup

- Whisper executable: `C:\Tools\whisper\release\whisper-cli.exe`
- Whisper model: `%APPDATA%\LocalAI\tools\whisper\models\ggml-base.en.bin`

## Workspace / Curator Notes

- Brain data loads from the current local workspace managed by the app.
- Curator staged packages are read from `curator/staged/` under the active workspace path.
- Packages added outside the app appear in the Curator Inbox after refresh.
- The external Cowork automation setup is documented in [COWORK_CURATOR_AUTOMATION_SPEC.md](C:/Users/pento/Desktop/ModernClawBase/COWORK_CURATOR_AUTOMATION_SPEC.md) so it can be rebuilt if the scheduled task is lost.

### Active Workspace Path Matters

Important current behavior:

- the Tauri backend resolves memory services from the active workspace path stored in the app database
- that means the live Curator inbox may read from `%APPDATA%\LocalAI\agents\<active-agent>\curator\staged\` instead of `%APPDATA%\LocalAI\curator\staged\`
- the same applies to imports into `knowledge/`

Practical rule for automation:

- Curator automation must resolve the active workspace dynamically
- do not assume the top-level `LocalAI\curator\` folders are the live app inbox
- if the active brain is `joe`, the live Curator folders are under `%APPDATA%\LocalAI\agents\joe\curator\`
- if the active brain changes, the automation should follow that workspace automatically
- the automation also needs access to `%APPDATA%\com.localai.app\data.db` so it can resolve the active workspace from the live database

## Local Data Location

The app uses the `LocalAI` app-data root for runtime files, including:

- `SOUL.md`
- `USER.md`
- `MEMORY.md`
- `memory/`
- `knowledge/`
- `curator/`
- `tools/`

When managed agent workspaces are in use, active runtime files may instead live under:

- `%APPDATA%\LocalAI\agents\<active-agent>\`

## Current Model Stack

- primary tested baseline: `gemma4:e4b`
