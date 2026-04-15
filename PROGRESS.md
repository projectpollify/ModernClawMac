# ModernClaw Direct Engine Mac Progress

## Mission

Turn this project into a direct-engine Mac edition that no longer depends on LM Studio for local chat.

Initial target:

- direct `llama.cpp` server integration
- same ModernClaw workspace, memory, and chat experience
- local-first setup with fewer external app dependencies

## Current Status

This project folder is a clean independent copy of the working `ModernClawMac` app.

What is true right now:

- the copied app still assumes LM Studio on macOS
- the backend already has a provider seam in Rust
- setup, status, and model-management flows still contain LM Studio-specific assumptions
- the direct-engine migration has started

## Locked Decisions

- first direct engine target: `llama.cpp`
- not starting with MLX-first integration
- keep the experiment in this separate project folder
- migrate in phases instead of rewriting the whole app at once

## Phase Plan

### Phase 1: Backend Foundation

- add a `LlamaCppService`
- define its local base URL and request format
- wire it into the Rust provider layer
- keep the app buildable while the UI still references LM Studio

### Phase 2: Engine Lifecycle

- decide how ModernClaw starts or connects to `llama-server`
- define expected local port and startup instructions
- expose clearer direct-engine status and startup errors

### Phase 3: Frontend Provider Migration

- replace LM Studio naming in provider config
- replace port `1234` assumptions
- update setup flow and status panels for the direct engine

### Phase 4: Model Strategy

- decide the first model-loading path
- likely options:
  - manual GGUF path
  - managed local models folder
  - full in-app model manager

Current recommendation:

- start with a managed local models folder after the first direct chat path works

### Phase 5: Cleanup

- remove or quarantine LM Studio-specific code
- simplify copy, onboarding, and model surfaces
- document the new direct-engine bring-up flow

## Started

- created this direct-engine project copy from the working ModernClawMac app
- confirmed the copied project contains the full app source
- confirmed the provider seam is real and reusable
- began Phase 1 by adding a dedicated `llama.cpp` service foundation
- switched the macOS default provider path from LM Studio to the direct `llama.cpp` service
- added direct-engine settings for the `llama-server` executable and GGUF model path
- installed `llama.cpp` on the M5 Mac through Homebrew
- verified a live `llama-server` run on port `8080` using the existing Gemma 4 E4B GGUF and multimodal projector

## Next Up

1. Add `LlamaCppService` beside the existing provider implementations.
2. Connect it to the Rust provider layer.
3. Decide when to flip macOS from LM Studio-backed to direct-engine-backed behavior.

## Working Rule

Keep this file current as the source of truth for the direct-engine experiment.
