# ModernClaw — Domain Expert Knowledge Base
**Status:** Initialized | Awaiting Full Data Integration
**Purpose:** Primary reference document for ModernClaw AI domain expertise
**Last Updated:** 2026-04-04

---

## 1. Origin & Intent

This document was created following a planning session conducted on a Windows machine using a locally installed instance of **Gemma 4** (Google's open-weight language model). During that session, the strategic intent for ModernClaw's AI integration was defined and refined.

The core decision made during that session:

> **This AI (Claude, operating within the Cowork environment) will become the primary domain expert for ModernClaw — ingesting all settings data, feature specifications, and configuration essentials — and will serve two simultaneous functions: internal development expert and external customer-facing AI agent.**

---

## 2. What ModernClaw Is

ModernClaw is a proprietary application built and operated by the same team behind BC Cutting Board Guys Inc. It is related to the **OpenClaw/Clawdbot AI agent infrastructure** already in use, and represents a more refined, productized evolution of that system.

### Known Infrastructure Context (from existing CLAUDE.md):

- **Gateway config:** `~/.openclaw/openclaw.json`
- **Gateway port:** 18898
- **Restart command:** `openclaw gateway restart` or `openclaw gateway --force`
- **Active agents:**
  - `alfred` — powered by `google/gemini-2.0-flash`, accessible via Telegram bot (alfred)
  - `jarvis` — powered by `anthropic/claude-sonnet-4-5`, accessible via Telegram bot (jarvis)
- **Agent soul/identity files:** `/home/shawn/projects/cbgwebapp/agents/<agent-id>/`
  - Each agent has: `AGENTS.md`, `IDENTITY.md`, `SOUL.md`, `TELEGRAM.md`, `USER.md`, and a `memory/` directory

ModernClaw appears to be the named product layer sitting above or alongside OpenClaw — the productized, customer-facing (and sellable) version of this AI agent infrastructure.

---

## 3. Strategic Role of This Document

### 3.1 Internal Expert Function
When detailed ModernClaw data is provided, this document (and the AI reading it) will serve as the **single source of truth** for:

- Feature capabilities and limitations
- Configuration settings and their impacts
- Dependencies between settings and modules
- Troubleshooting logic and known edge cases
- Development decisions and rationale

### 3.2 External Customer Service Function
The same knowledge will power a **website AI customer service agent** capable of:

- Answering user questions about ModernClaw features in plain language
- Guiding users through configuration and setup
- Explaining what settings do without requiring technical knowledge
- Providing accurate, consistent answers drawn from structured data

This dual-use architecture means every piece of data entered here must be written to serve **both a developer and an end-user** — technical precision with accessible explanations.

---

## 4. Data Integration Schema

When ModernClaw specifications are provided, they will be structured using the following hierarchy. This is the schema defined during the Gemma 4 planning session.

### 4.1 Settings Data Format

```
[Feature Name]
  └── [Setting Key]
        ├── Default Value: [value]
        ├── Accepted Values: [range / options]
        ├── User Impact: [plain-language description of what this changes]
        └── Developer Notes: [technical context, dependencies, warnings]
```

**Example:**

```
[Agent Routing]
  └── dmPolicy
        ├── Default Value: "allowlist"
        ├── Accepted Values: "allowlist" | "open" | "disabled"
        ├── User Impact: Controls who can send direct messages to the bot.
        └── Developer Notes: Must match binding accountId in openclaw.json.
                              "allowlist" restricts to approved Telegram user IDs only.
```

---

### 4.2 Features Data Format

```
[Module Name]
  └── [Function Name]
        ├── Description: [what it does]
        ├── Requires Config: [which settings must be set]
        ├── Dependencies: [other modules or services needed]
        └── User-Facing Summary: [how to explain this to a customer]
```

---

### 4.3 Configuration Essentials Format

```
[Rule Name]
  ├── Applies To: [which settings / modules]
  ├── Constraint: [what rule governs this]
  ├── If Violated: [what breaks or what error occurs]
  └── Resolution: [how to fix it]
```

**Example (from known config):**

```
[Agent-Telegram Binding Rule]
  ├── Applies To: bindings[].match.accountId, channels.telegram.accounts keys
  ├── Constraint: accountId in bindings must exactly match the key name under
                  channels.telegram.accounts
  ├── If Violated: Messages sent to Bot A may route to wrong agent or fail entirely
  └── Resolution: Ensure both values are identical strings (case-sensitive)
```

---

## 5. Dual-Use Translation Principle

Every technical setting or feature documented here must include a **plain-language version**. This is not optional — it is the mechanism that makes this document useful for the customer service agent.

| Layer | Audience | Tone |
|---|---|---|
| Developer Notes | Internal team, builders | Precise, technical, concise |
| User Impact / User-Facing Summary | End customers, non-technical | Clear, friendly, benefit-focused |

When writing user-facing explanations, assume the reader:
- Has never seen a config file
- Understands what they want to accomplish, not how it's built
- Needs to know "what does this do for me" not "how does this work"

---

## 6. Relationship to Business Exit Strategy

ModernClaw is not just a tool — **it is part of the asset being sold.**

As documented in `CLAUDE.md`, the AI agent infrastructure is a core component of the BC Cutting Board Guys Inc. exit package. ModernClaw, as the productized layer, contributes directly to:

- **Reduced owner dependency** — the system operates and guides users without requiring Shawn's involvement
- **Transferability** — a buyer can inherit a fully documented, AI-powered operational system
- **Valuation premium** — proprietary technology with documented expertise commands higher multiples
- **Scalability** — the same customer service AI that handles one province can handle four

This document, once fully populated, is a **due diligence asset** — something a buyer reviews and concludes: *"This business runs itself."*

---

## 7. Data Upload Roadmap

The following data categories are expected to be provided and integrated into this document over time:

- [ ] Full ModernClaw feature list
- [ ] Settings schema (all keys, values, defaults)
- [ ] Configuration rules and dependency map
- [ ] Agent identity and behavior specifications
- [ ] Telegram channel configuration details
- [ ] Known edge cases and troubleshooting guides
- [ ] Customer-facing FAQ (to be derived from above)
- [ ] Website AI agent prompt/persona guidelines

---

## 8. How to Use This Document

**When adding data:**
Use the schemas in Section 4. Paste raw data and I will structure it into the correct format.

**When asking questions:**
Reference the module or setting name. I will cross-reference everything in this document first before drawing on general knowledge.

**When building the customer service agent:**
The "User Impact" and "User-Facing Summary" fields throughout this document are the source material. They can be extracted directly into the agent's knowledge base.

---

*This document is a living record. Every piece of ModernClaw data provided will be integrated here. It grows with the product.*
