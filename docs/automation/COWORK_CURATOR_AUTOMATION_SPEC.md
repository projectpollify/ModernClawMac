# Cowork Curator Automation Spec

## Purpose

This document captures the Curator automation setup that lives outside the app repo so it can be rebuilt if the Cowork scheduled task is lost, broken, or needs to be recreated on another machine.

ModernClaw depends on this automation for request intake, source gathering, package preparation, and staging into the live Curator inbox.

If the automation breaks, this document is the recovery source of truth.

Important product direction:

- the intended user experience is one plain-text input box
- Curator should act as the intake interpreter
- structured request files remain a compatibility and operator-control path

## What The Automation Does

The Curator automation is a recurring Cowork task that:

1. resolves the active ModernClaw workspace dynamically
2. scans that workspace's `curator/requests/` folder
3. parses the next eligible request form
4. switches behavior based on request source mode
5. prepares a Curator package
6. stages that package into the active workspace's `curator/staged/`
7. archives or marks the original request as processed

## Why Dynamic Resolution Matters

The app does not always read from the top-level:

`%APPDATA%\LocalAI\curator\`

When active agent workspaces are in use, the live Curator inbox is instead under:

`%APPDATA%\LocalAI\agents\<active-agent>\curator\`

That means the automation must follow the active workspace path stored in the app database rather than assuming one fixed folder.

## Required External Mounts

The Cowork task must have access to both of these mounted folders:

- `%APPDATA%\LocalAI`
- `%APPDATA%\com.localai.app`

Why both are required:

- `LocalAI` contains the runtime workspace folders
- `com.localai.app` contains the SQLite database that identifies the active agent and workspace path

If either mount is missing, the task may appear to run but will not be able to route packages correctly.

## Active Workspace Resolver

The task must resolve the active workspace on each run.

### Database

Current database location:

`%APPDATA%\com.localai.app\data.db`

### Required data lookup

The task must:

1. read `active_agent_id` from the settings table
2. look up that agent in the agents table
3. read the corresponding `workspace_path`

### Result

That `workspace_path` becomes the live root for:

- `curator/requests/`
- `curator/in-progress/`
- `curator/staged/`
- `curator/approved/`
- `curator/rejected/`
- `curator/archive/`
- `knowledge/`

## Example Runtime Paths

If the active agent is `joe`, the live runtime folders become:

- `%APPDATA%\LocalAI\agents\joe\curator\requests\`
- `%APPDATA%\LocalAI\agents\joe\curator\in-progress\`
- `%APPDATA%\LocalAI\agents\joe\curator\staged\`
- `%APPDATA%\LocalAI\agents\joe\curator\archive\`
- `%APPDATA%\LocalAI\agents\joe\knowledge\`

Do not stage packages into the top-level `LocalAI\curator\staged\` folder unless the top-level workspace is truly the active workspace.

## Request Intake Contract

Primary intake model:

- user free-text request
- Curator interprets intent
- Curator creates an internal request object

Compatibility intake model:

- Markdown request file parsed with explicit fields

The automation consumes Markdown request files based on:

- [CURATOR_INTAKE_INTERPRETER_SPEC.md](CURATOR_INTAKE_INTERPRETER_SPEC.md)
- [CURATOR_REQUEST_TEMPLATE.md](CURATOR_REQUEST_TEMPLATE.md)
- [CURATOR_REQUEST_PARSER_SPEC.md](CURATOR_REQUEST_PARSER_SPEC.md)

The parser must support:

- `web research`
- `NotebookLM`
- `manual`

### Source mode rules

- if `Source` is `NotebookLM`, run NotebookLM mode
- if `NotebookLM Notebook URL` is present, run NotebookLM mode even if `Source` was omitted
- if `Source` is `manual`, run manual transformation mode
- otherwise default to web research mode

## Package Output Contract

Each completed request should produce:

```text
<active-workspace>\curator\staged\<package-folder>\
  curated-knowledge.md
  meta.json
  assets\            (optional)
```

### `meta.json`

Expected common fields:

- `title`
- `summary`
- `source`
- `tags`
- `request_topic`
- `created_at`

Expected extra NotebookLM fields when applicable:

- `notebook_url`
- `notebook_output_type`

## Processing Flow

### Step 1: Scan requests

- read the active workspace's `curator/requests/`
- ignore templates and helper files such as `TEMPLATE-*`
- choose one real request at a time

### Step 2: Claim the request

- move the request into `curator/in-progress/`
- or create a lock/working folder that clearly marks ownership

### Step 3: Parse the form

- normalize the request using the parser spec
- validate required fields for the resolved source mode

### Step 4: Execute by source mode

#### Web mode

- perform internet research
- prefer strong sources
- synthesize into ModernClaw knowledge format

#### NotebookLM mode

- open the provided notebook
- extract the requested NotebookLM output type
- capture raw results
- transform into ModernClaw knowledge format

#### Manual mode

- transform the supplied content without assuming web research
- only browse if the request explicitly calls for it

### Step 5: Stage package

- write the package into `curator/staged/`
- ensure package contents are reviewable and importable

### Step 6: Archive the request

- move the completed request into `curator/archive/`
- clear any in-progress lock or working folder

## Failure Behavior

If parsing or execution fails:

- do not silently produce a broken package
- preserve the request for inspection
- record a short failure note or move it to a failed state

Typical failure cases:

- missing NotebookLM URL
- missing NotebookLM output type
- unsupported output type
- notebook inaccessible
- browser not logged into the needed Google account
- missing Curator mount or app DB mount

## Current Known Risks

- Cowork session mount paths can change between runs
- the app database path is external to the repo
- NotebookLM UI changes may break extraction
- browser login state may expire
- if the task writes to the wrong workspace, the app inbox will appear empty

## Rebuild Checklist

If the Cowork task disappears, rebuild it with these requirements:

1. Create a recurring Curator automation in Cowork.
2. Mount `%APPDATA%\LocalAI`.
3. Mount `%APPDATA%\com.localai.app`.
4. Resolve the active workspace dynamically from `data.db`.
5. Scan `<active-workspace>\curator\requests\`.
6. Parse requests using [CURATOR_REQUEST_PARSER_SPEC.md](CURATOR_REQUEST_PARSER_SPEC.md).
7. Stage packages into `<active-workspace>\curator\staged\`.
8. Archive processed requests into `<active-workspace>\curator\archive\`.
9. Ignore `TEMPLATE-*` request files.
10. Preserve package compatibility with ModernClaw import expectations.

## Minimum Prompt Requirements

Any recreated Cowork task prompt should explicitly tell the agent to:

- resolve the active workspace dynamically from the app database
- never assume the top-level `LocalAI\curator\` path is the live inbox
- parse request forms before acting
- switch among `web`, `NotebookLM`, and `manual` modes
- emit `curated-knowledge.md` and `meta.json`
- leave review authority with the user through Curator Inbox

## Recovery Rule

If the automation behavior changes, update this file immediately.

This document should stay accurate enough that a new Curator automation can be rebuilt from it without guesswork.
