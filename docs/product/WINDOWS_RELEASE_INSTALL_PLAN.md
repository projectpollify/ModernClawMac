# Windows Release Install Plan

## Goal

Ship a normal Windows installer for ModernClaw that gets a non-developer from download to first chat with as little manual setup as possible.

This is a release-installer plan, not a repo-from-source plan.

## Product Standard

The end-user release path should not require:

- Node.js
- Rust
- cargo
- npm
- repo cloning

The release path should feel like:

1. Download ModernClaw installer.
2. Install ModernClaw.
3. Let the app help the user get Ollama running.
4. Click `Install Recommended Model`.
5. Start chatting.

## Current Reality

What already exists:

- Tauri app packaging is enabled in [local-ai/src-tauri/tauri.conf.json](../../local-ai/src-tauri/tauri.conf.json)
- Windows bundle targets already include `nsis` and `msi`
- the app already has in-app setup actions for:
  - opening the Ollama download page
  - attempting to start Ollama
  - installing the recommended model
  - initializing workspace files
- the app now highlights the next required setup action

What is not done yet:

- no release-focused Windows installer configuration
- no installer branding or release-oriented Windows metadata cleanup
- no defined packaged-installer test flow
- no bundled voice/tool strategy for release

## What We Can Bundle Now With Tauri

Based on the current local Tauri configuration and schema available in `@tauri-apps/cli`:

- the compiled ModernClaw app itself
- frontend assets
- icons and app metadata
- Windows installers via `nsis` and `msi`
- WebView2 installation handling through Tauri's Windows `webviewInstallMode`
- extra bundled resources via `bundle.resources`
- extra shipped helper binaries via `bundle.externalBin`
- installer customization through NSIS hooks
- MSI customization through WiX settings

This means the Tauri release installer can own:

- app installation
- shortcut creation
- install directory choices
- launch-after-install behavior
- part of the Windows runtime dependency experience
- optional helper scripts or support binaries we choose to ship

## What Should Stay External For First Release

These should stay external or in-app for the first Windows release:

- Ollama itself as a separately installed dependency
- model downloads such as `gemma4:e4b`
- Piper and Whisper binaries
- Piper voice files
- Whisper model files

Reasoning:

- Ollama is its own runtime with service/start/update behavior
- model weights are large and should stay user-visible and on-demand
- bundling voice binaries and models will bloat the installer and slow the first release
- voice is already optional in the product

## First Release Recommendation

Release a bootstrap-style Windows installer, not an everything-bundled installer.

That first installer should:

1. Install ModernClaw itself.
2. Ensure WebView2 is handled correctly during install.
3. Launch ModernClaw after install.
4. Let ModernClaw guide the user through Ollama install/start and model download.

This is the fastest credible path to shipping.

## Installer Strategy

### Installer Type

Use `nsis` as the primary Windows installer target for the first release.

Why:

- it is already supported by the current Tauri config
- it gives us practical hook points for release polish
- it is a better fit for guided Windows installer behavior than treating MSI as the primary consumer installer

Keep `msi` available as a secondary artifact for environments that prefer it.

### WebView Strategy

Explicitly configure Windows `webviewInstallMode` in Tauri instead of relying on defaults by implication.

First-release recommendation:

- use Tauri's bootstrapper-based WebView2 install flow
- do not ship a fixed embedded WebView2 runtime in v1 unless testing proves bootstrapper installs are failing in the field

Reason:

- fixed WebView2 adds major installer size
- bootstrapper keeps the installer lighter
- this is good enough for a connected Windows release path

### Ollama Strategy

For v1, do not try to silently install Ollama inside the ModernClaw installer.

Instead:

- detect the missing dependency in-app
- offer `Download Ollama`
- offer `Start Ollama`
- show the next step clearly

Optional installer polish for v1.1:

- add a post-install prompt that opens the Ollama download page
- or add an installer checkbox like `Open Ollama Download After Setup`

### Model Strategy

Keep model installation in-app.

The installer should not bundle `gemma4:e4b`.

Instead:

- the app should guide the user to `Install Recommended Model`
- onboarding and `Setup` should remain the single source of truth for machine readiness

### Voice Strategy

Keep voice completely out of the first Windows release critical path.

First release rule:

- chat must be easy
- voice can come later

## Exact First Release User Flow

This is the target step-by-step experience for a first-time Windows user.

1. User downloads the ModernClaw installer.
2. User runs the installer.
3. Installer installs ModernClaw and handles WebView2 requirements.
4. Installer offers `Launch ModernClaw`.
5. ModernClaw opens into onboarding or the `Setup` surface.
6. If Ollama is missing, ModernClaw clearly says `Get Ollama running first`.
7. User clicks `Download Ollama` if needed.
8. User installs Ollama.
9. User returns to ModernClaw and clicks `Start Ollama`.
10. ModernClaw confirms Ollama is responding.
11. ModernClaw presents `Install Recommended Model`.
12. User installs `gemma4:e4b`.
13. ModernClaw confirms the workspace files are ready.
14. User reaches chat.

## Exact Build And Packaging Work

### Phase 1: Release Installer Baseline

1. Add Windows-specific release config under Tauri.
2. Set explicit NSIS and WebView2 installer options.
3. Clean up Windows bundle metadata:
   - app identifier
   - publisher
   - descriptions
   - copyright
4. Produce a testable NSIS installer artifact.

### Phase 2: Installer UX Polish

1. Ensure the installer launches ModernClaw after install.
2. Add release notes text and installer branding.
3. Decide whether to add a post-install Ollama prompt through NSIS hooks.
4. Make Setup the default recovery surface when required setup is incomplete.

### Phase 3: Release Validation

1. Test on a clean Windows machine or VM.
2. Validate install without Node.js or Rust.
3. Validate WebView2 behavior.
4. Validate Ollama installation detour and return path.
5. Validate recommended model install.
6. Validate first chat.

## Work Breakdown

### Track A: Tauri Windows Packaging

Deliverables:

- explicit Windows release config
- NSIS-first installer settings
- reproducible `tauri build` output for Windows release

### Track B: Installer Experience

Deliverables:

- launch-after-install behavior
- release metadata polish
- optional NSIS post-install helper behavior

### Track C: App Setup Experience

Deliverables:

- keep the current `next required action` flow stable
- ensure Setup is the obvious recovery path after install
- make first-chat completion easy to verify

### Track D: QA

Deliverables:

- clean-machine test script
- pass/fail checklist
- issue list ranked by install severity

## Acceptance Criteria For V1 Windows Release

We can ship when all of these are true:

- a user can install ModernClaw without Node.js or Rust
- the installer completes successfully on a clean Windows machine
- the app clearly guides the user to the next required action
- the Ollama dependency does not feel hidden or mysterious
- the recommended model can be installed from inside the app
- the workspace files initialize automatically
- the user can send a first chat message

## Risks

### Risk: Ollama remains the biggest external dependency

Mitigation:

- treat this as an explicit part of the first-run flow
- never hide the dependency
- keep the path clear and deliberate

### Risk: WebView2 issues block some Windows installs

Mitigation:

- set explicit Tauri Windows install mode
- test on clean machines early
- switch to a heavier fixed-runtime strategy only if needed

### Risk: Voice scope delays release

Mitigation:

- keep voice out of release-critical install
- label it optional everywhere

### Risk: Trying to bundle everything creates a giant fragile installer

Mitigation:

- ship the bootstrap installer first
- add optional dependency packs later

## Immediate Next Actions

1. Add a Windows-specific Tauri config for release installer behavior.
2. Choose NSIS as the primary Windows artifact.
3. Set explicit WebView2 install mode in config.
4. Clean up installer metadata and release descriptions.
5. Produce the first Windows release build.
6. Run a clean-machine install test.

## Decision Summary

The first release should be:

- packaged
- Windows-first
- NSIS-first
- WebView2-aware
- Ollama-assisted, not Ollama-bundled
- model-download-in-app
- voice-optional

That is the fastest path from today to a real release.
