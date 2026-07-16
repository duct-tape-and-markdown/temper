# Projection sample: `.claude/skills/harness-meta/SKILL.md`

The intake skill from `skills.ts`. The final line is the reference
template (`"{prose}: {cite.path}."`) rendered over the resolved edge —
`{cite.path}` is the target's projection path relative to this member's
own projection, so the citation is actionable as written.

```markdown
---
# temper: managed projection — edit the owning .temper/ module, never this file.
name: "harness-meta"
description: "Intake & maintenance for the harness itself — rules, CLAUDE.md, skills. Use when adding, moving, demoting, or auditing harness guidance: 'where does this belong', 'add a directive', 'demote this convention', 'harness audit', 'is this rule earning its place'."
allowed-tools: ["Read", "Write", "Edit", "Grep", "Glob", "Bash", "AskUserQuestion"]
---
# Harness Intake

Run this when the harness itself is the work — new guidance to place, a
rule to demote, an audit.

The harness is a projection of the temper program at `.temper/`: edit
the owning module and run `temper emit` — a direct edit to `CLAUDE.md`
or `.claude/` is drift, and the guard refuses it. `temper check` holds
the structure.

The doctrine — what belongs in a harness at all, the filing algorithm,
and the judgments no clause can hold. Read it, file accordingly, and let
the gate verify the wiring: docs/economy.md.
```
