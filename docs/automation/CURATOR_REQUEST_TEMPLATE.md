# Curator Knowledge Request

This template is a compatibility and advanced-control format.

Normal product UX should not require the user to fill this out directly. In the intended flow, the user types a plain-language request and Curator interprets it into an internal request object.

## Source
[web research / NotebookLM / manual]
[This determines which Curator workflow runs. If omitted, Curator should default to web research unless a NotebookLM URL is present.]

## NotebookLM Notebook URL
[Paste the full notebook URL here. Only required if Source is NotebookLM.]

## NotebookLM Output Type
[Briefing Doc / FAQ / Study Guide / Timeline / Mind Map / Chat Q&A]
[Only required if Source is NotebookLM. Prefer one output type per request.]

## Topic
[Main subject]

## Goal
[What knowledge should be produced and why]

## Key Questions
- [Question 1]
- [Question 2]
- [Question 3]

## Request Topic
[Short slug-like label for Curator Inbox review]

## Intended Use In ModernClaw
[How this knowledge should help the app, user, memory system, or future answers]

## Depth
[Quick overview / Standard summary / Deep dive]

## Source Preferences
[Preferred sources, domains, or source types. For web research mode, list what Curator should trust most.]

## Avoid
[Things to exclude: weak sources, speculation, outdated material, marketing fluff, unsupported claims, etc.]

## Special Instructions
[Any formatting, tone, packaging, or prioritization rules]

## Output Requirements
- Create one staged Curator package folder
- Include `curated-knowledge.md`
- Include `meta.json`
- Keep the knowledge distilled, reviewable, and ready for ModernClaw import
- Split into multiple packages if the topic is too large for one focused file

## Packaging Notes
- If Source is `NotebookLM`, set `meta.json` `source` to `NotebookLM`
- If Source is `NotebookLM`, include `notebook_url` in `meta.json`
- If Source is `NotebookLM`, include the requested NotebookLM output type in package notes or metadata
