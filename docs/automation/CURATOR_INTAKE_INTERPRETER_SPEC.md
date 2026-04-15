# Curator Intake Interpreter Spec

## Purpose

This document defines the real Curator user experience:

- the user types plain language into one box
- Curator interprets intent
- Curator builds an internal request object
- Curator routes the work to the correct acquisition mode

The user should not be expected to understand Curator internals such as `Source`, `NotebookLM Output Type`, or package metadata.

Those are system concerns, not user-facing workflow requirements.

## Product Principle

The user should do one thing:

- describe what they want

Curator should do the rest:

- infer the topic
- infer the goal
- detect the likely source path
- create the internal request object
- execute the right workflow

## User-Facing Intake

The user-facing intake surface is one plain-text input box.

Examples:

- `Research practical earthquake water storage and turn it into usable knowledge for Joe.`
- `Use my NotebookLM notebook on React Server Components and turn it into clean ModernClaw knowledge: https://notebooklm.google.com/notebook/...`
- `Turn these pasted notes into a clean knowledge file about local voice setup.`

The user does not need to fill out:

- `Source`
- `NotebookLM Notebook URL`
- `NotebookLM Output Type`
- `Request Topic`
- `Depth`

Curator should infer or generate those internally.

## Curator Responsibilities

Curator has two phases:

### Phase 1: Intake interpretation

Curator reads the user's plain-language request and determines:

- what the user actually wants
- whether the best path is:
  - `web`
  - `notebooklm`
  - `manual`
- what the topic is
- what useful output should be produced
- what the request slug should be
- what important constraints or preferences are present

### Phase 2: Execution

Curator uses the interpreted request object to run the appropriate workflow and produce a staged Curator package.

## Hidden Internal Request Object

Curator should transform user input into an internal object like this:

```json
{
  "source_mode": "web",
  "topic": "",
  "goal": "",
  "request_topic": "",
  "key_questions": [],
  "intended_use": "",
  "depth": "standard",
  "source_preferences": "",
  "avoid": "",
  "special_instructions": "",
  "notebook_url": "",
  "notebook_output_type": "",
  "raw_user_request": ""
}
```

This object is internal.

It exists so Curator can operate deterministically after interpreting free-text input.

## Routing Signals

Curator should use clear routing signals when interpreting plain text.

### Route to `notebooklm`

If any of these are present:

- a NotebookLM URL
- explicit language like:
  - `use my NotebookLM notebook`
  - `pull from NotebookLM`
  - `extract from NotebookLM`

### Route to `manual`

If the user provides or clearly refers to supplied material such as:

- pasted notes
- attached text
- copied source content
- `turn this into knowledge`
- `rewrite these notes`

### Route to `web`

If the user is asking for knowledge on a topic and no NotebookLM or manual-material signal is present.

This should be the default.

## Default Inference Rules

Curator should infer these fields when possible:

### `topic`

A concise version of the main subject.

### `goal`

A short statement of what knowledge should be produced and why.

### `request_topic`

A slug-like label used for package naming and inbox clarity.

Example:

- `react-server-components`
- `earthquake-water-storage`
- `local-voice-setup`

### `depth`

Default to `standard` unless the user clearly asks for:

- `quick`, `brief`, `short` => `quick`
- `deep`, `thorough`, `detailed` => `deep`

Otherwise use `standard`.

## NotebookLM Output Type Selection

If Curator routes to NotebookLM mode, it should also choose the best NotebookLM output type.

Preferred defaults:

- explanatory topic => `study_guide`
- broad summary request => `briefing_doc`
- question-oriented request => `faq`
- event history request => `timeline`

If the user explicitly asks for a certain NotebookLM output type, honor that.

If Curator is not confident which NotebookLM output type fits best, prefer:

- `briefing_doc`

## Confidence And Escalation

Curator should not ask the user follow-up questions for normal requests if a reasonable inference can be made.

Curator should proceed when confidence is high enough.

Examples of high-confidence cases:

- a NotebookLM URL is present
- the topic is obvious
- the user clearly wants research
- the user clearly supplied transformable material

Curator should only escalate when the request is ambiguous enough that proceeding would likely waste effort.

Examples:

- multiple unrelated topics in one request
- unclear whether pasted content is reference material or just commentary
- a malformed NotebookLM URL that may be incomplete

## Compatibility Path

The Markdown Curator request template still exists, but it is no longer the primary UX.

Use it for:

- debugging
- operator testing
- direct automation inputs
- advanced manual control

The normal product path should be:

```text
user free-text request
-> Curator interprets intent
-> internal request object
-> routed execution
-> staged package
```

## Success Condition

The intake interpreter succeeds when:

- the user only had to describe what they wanted
- Curator correctly inferred the acquisition mode
- Curator produced a clean internal request object
- the downstream curation workflow could proceed without user form-filling
