# Rosie Review Template

Use this template for `rosie-review.md` inside each staged Curator package.

```md
# Rosie Review

## Verdict
[approve / revise / reject]

## Confidence
[0.00 to 1.00]

## Summary
[Short explanation of the overall judgment]

## What Checks Out
- [Strength 1]
- [Strength 2]
- [Strength 3]

## Issues Found
- [Issue 1]
- [Issue 2]
- [Issue 3]

## Recommended Action
[Approve for import / Revise before import / Reject package]

## Required Changes
- [Change 1]
- [Change 2]
- [Change 3]

## Knowledge Fit
[knowledge / memory-candidate / unclear]

## Duplicate / Conflict Notes
- [Possible overlap or conflict 1]
- [Possible overlap or conflict 2]

## Reviewer Notes
[Any additional caution, source-quality note, or user-facing explanation]
```

## Usage Notes

- `Required Changes` may be omitted when the verdict is `approve`.
- `Duplicate / Conflict Notes` may be omitted when no overlap is found.
- `Knowledge Fit` should normally be `knowledge`.
- If the content seems more appropriate for memory, do not write to `MEMORY.md`; note it as a separate recommendation instead.

## JSON Pairing

When possible, pair the Markdown review with:

- `rosie-review.json`

Recommended JSON fields:
- `verdict`
- `confidence`
- `summary`
- `checks_passed`
- `issues`
- `required_changes`
- `knowledge_fit`
- `memory_write_recommended`
- `duplicate_candidates`

## Review Standard

Rosie should favor:
- clarity over vagueness
- specific revision guidance over general criticism
- trust preservation over fast approval
- compact, high-signal reasoning over long commentary
