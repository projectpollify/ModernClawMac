# Contributing to ModernClaw

Thanks for contributing to ModernClaw.

This repo is currently centered on the desktop app in [`local-ai/`](local-ai/), with supporting documentation organized under [`docs/`](docs/).

## Before You Start

1. Read [README.md](README.md) for the project overview.
2. Read [docs/runbooks/RUNBOOK.md](docs/runbooks/RUNBOOK.md) for bring-up and validation notes.
3. Check [docs/product/PROGRESS.md](docs/product/PROGRESS.md) for current priorities and open questions.

## Development Setup

### Requirements

- Node.js 20+
- Rust stable toolchain
- Ollama installed locally

### Start The App

```powershell
cd "C:\path\to\ModernClaw\local-ai"
npm install
npm run tauri:dev
```

## Project Structure

- `local-ai/`: Tauri desktop app
- `docs/product/`: product scope, progress, and implementation plans
- `docs/runbooks/`: validation and operating notes
- `docs/automation/`: Curator-related specs and prompts
- `docs/research/`: research and comparison reference material

## Contribution Expectations

- Keep changes scoped and reviewable.
- Prefer fixing root causes over adding one-off workarounds.
- Update docs when product behavior changes.
- Keep Windows install and clean-machine setup in mind for user-facing changes.
- Do not commit generated build artifacts or local machine state.

## Validation

Run these before opening a pull request when your change touches the app:

```powershell
cd "C:\path\to\ModernClaw\local-ai"
npm run build
cd "C:\path\to\ModernClaw\local-ai\src-tauri"
cargo check
```

If your change affects installer flow, onboarding, setup, or packaging:

- note whether it was tested on a clean or previously used machine
- describe any Ollama/model assumptions
- call out any manual verification steps in the PR

## Pull Requests

- Link the related issue if there is one.
- Explain user-facing behavior changes clearly.
- Mention docs updates explicitly.
- Keep screenshots or short repro notes for UI/setup changes.

## Security

Please do not open public issues for potential security vulnerabilities. See [SECURITY.md](SECURITY.md).
