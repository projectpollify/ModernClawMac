# Curator Request Parser Spec

## Purpose

This document defines how Curator should parse a structured Markdown request form and decide which execution mode to run.

This is a compatibility and advanced-control path.

The primary user experience is defined in [CURATOR_INTAKE_INTERPRETER_SPEC.md](C:/Users/pento/Desktop/ModernClawBase/CURATOR_INTAKE_INTERPRETER_SPEC.md), where Curator interprets free-text user intent and builds the internal request object itself.

The parser here is for cases where Curator receives a pre-structured request file and must normalize it, validate it, and branch into the correct workflow.

## Supported Source Modes

Curator supports three source modes:

- `web research`
- `NotebookLM`
- `manual`

The parser should normalize source values case-insensitively and trim surrounding whitespace.

Valid normalized outputs:

- `web`
- `notebooklm`
- `manual`

## Supported Request Fields

The parser should recognize these headings:

- `Source`
- `NotebookLM Notebook URL`
- `NotebookLM Output Type`
- `Topic`
- `Goal`
- `Key Questions`
- `Request Topic`
- `Intended Use In ModernClaw`
- `Depth`
- `Source Preferences`
- `Avoid`
- `Special Instructions`
- `Output Requirements`
- `Packaging Notes`

If additional sections appear, preserve them as optional notes instead of failing.

## Normalized Request Object

The parser should produce a normalized object like this:

```json
{
  "source_mode": "web",
  "topic": "",
  "goal": "",
  "key_questions": [],
  "request_topic": "",
  "intended_use": "",
  "depth": "",
  "source_preferences": "",
  "avoid": "",
  "special_instructions": "",
  "notebook_url": "",
  "notebook_output_type": ""
}
```

## Parsing Rules

### 1. Heading extraction

- Read the file as Markdown
- Treat `##` headings as top-level request sections
- Capture the text that follows each heading until the next `##`
- Preserve multiline values
- Parse bullet lists under `Key Questions` into a string array

### 2. Source mode resolution

Use this decision order:

1. If `Source` is explicitly `NotebookLM`, set `source_mode = "notebooklm"`
2. Else if `NotebookLM Notebook URL` is present, set `source_mode = "notebooklm"`
3. Else if `Source` is explicitly `manual`, set `source_mode = "manual"`
4. Else default to `source_mode = "web"`

### 3. NotebookLM output type normalization

Accept these values:

- `Briefing Doc`
- `FAQ`
- `Study Guide`
- `Timeline`
- `Mind Map`
- `Chat Q&A`

Normalize them into stable internal identifiers:

- `briefing_doc`
- `faq`
- `study_guide`
- `timeline`
- `mind_map`
- `chat_qa`

If the form requests multiple NotebookLM output types, the parser should reject the request for now and ask for one output type per request.

## Validation Rules

### Required for all modes

- `Topic`
- `Goal`
- `Request Topic`

### Required for `web`

No additional required fields.

Optional but useful:

- `Key Questions`
- `Source Preferences`
- `Avoid`

### Required for `notebooklm`

- `NotebookLM Notebook URL`
- `NotebookLM Output Type`

Optional but useful:

- `Key Questions`
- `Special Instructions`

### Required for `manual`

No additional required fields, but the request should clearly describe the transformation goal and what source material is being supplied manually.

## Execution Branching

After parsing and validation, Curator should switch behavior like this:

### `web`

- perform internet research
- evaluate sources
- synthesize findings
- produce a standard Curator package

### `notebooklm`

- open the provided NotebookLM notebook
- extract the requested NotebookLM output type
- capture the raw output
- transform it into ModernClaw knowledge format
- produce a standard Curator package

### `manual`

- do not browse unless the request explicitly asks for browsing
- transform the provided material into ModernClaw knowledge format
- produce a standard Curator package

## Failure Rules

The parser should stop and mark the request invalid if:

- `Source` is `NotebookLM` but the notebook URL is missing
- `Source` is `NotebookLM` but the output type is missing
- the NotebookLM output type is unsupported
- `Topic`, `Goal`, or `Request Topic` is missing

When invalid, Curator should not create a staged knowledge package.

Instead, it should produce a clear error note or move the request into a failed state with a short explanation.

## Package Metadata Expectations

For `web` requests, `meta.json` should usually include:

- `title`
- `summary`
- `source`
- `tags`
- `request_topic`
- `created_at`

For `NotebookLM` requests, `meta.json` should also include:

- `notebook_url`
- `notebook_output_type`

For `manual` requests, `meta.json` should set `source` to `manual` or another explicit manual-origin label.

## Example: Web Request

```json
{
  "source_mode": "web",
  "topic": "Local AI markdown knowledge packaging",
  "goal": "Create a short knowledge note about best practices for durable context files.",
  "key_questions": [
    "What makes a knowledge file useful for prompt context?",
    "How should large topics be split?"
  ],
  "request_topic": "knowledge-packaging",
  "intended_use": "Help ModernClaw ingest cleaner knowledge files.",
  "depth": "Quick overview",
  "source_preferences": "Official docs and durable technical references",
  "avoid": "SEO filler and speculation",
  "special_instructions": "Keep it concise",
  "notebook_url": "",
  "notebook_output_type": ""
}
```

## Example: NotebookLM Request

```json
{
  "source_mode": "notebooklm",
  "topic": "React Server Components",
  "goal": "Turn a NotebookLM study guide into a clean ModernClaw knowledge note.",
  "key_questions": [
    "How do server and client components differ?",
    "What are the main tradeoffs?"
  ],
  "request_topic": "react-server-components",
  "intended_use": "Feed architecture guidance into ModernClaw knowledge.",
  "depth": "Standard summary",
  "source_preferences": "",
  "avoid": "Marketing language",
  "special_instructions": "Keep it developer-friendly",
  "notebook_url": "https://notebooklm.google.com/notebook/...",
  "notebook_output_type": "study_guide"
}
```
