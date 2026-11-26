# 0101 — a built-in reads as the runtime reads

- **Date:** 2026-11-25 · **Status:** accepted

## Context

A stranger-gate probe of a real 297-member harness, and the plan questions
that followed, raised one class of problem in five forms:

- an unquoted `description:` containing `": "`: strict YAML rejects it, but
  Claude Code loads the skill (55 load faults, with 131 more cascading);
- an `@`-import path's delimiter: the docs give a backslash-escaped space
  and
  say nothing on trailing punctuation;
- agent names: a `:`, or a leading `-`, makes Claude Code skip the file;
- output styles: a documented surface with no kind;
- `.claude/worktrees/`: runtime state, which the docs say to gitignore.

John, 2026-11-25: think in classes, with these as instances.

## Decision

**A built-in reads its format as the runtime does** (`builtins.md`,
opening). Three rules apply in order:

- the documented behavior binds;
- where the docs are silent, a cited lab observation binds;
- where neither exists, the literal bytes bind, and the guidance names the
  uncertainty.

A document the runtime loads is never a load fault, and one it skips is.
The instances:

- Skills, commands and output styles whose YAML does not parse load with no
  fields set, as the docs say. The finding names the non-portable YAML, with
  the lab result in its guidance. Agents keep the fault, because they are
  skipped.
- An import path takes the documented backslash-space escape and keeps
  trailing punctuation literal. Its unbacked finding says that a trailing
  `.` is part of the path.
- The agent name contract refuses `:` anywhere and a leading `-`, by
  widening `allowedChars` with a negated charset and a leading position
  (one primitive widened, not two added).
- `output-style` becomes a kind (the coverage bar).
- `.claude/worktrees/` gets no fence. Discovery already honors the gitignore
  the docs ask for, and the finding's guidance quotes that advice.

## Rejected

- **Strict YAML everywhere.** It calls a loading member broken. The
  strictest-profile stance governs clauses, not whether a member exists.
- **Stripping trailing punctuation.** That invents parser behavior.

## Consequences

When plan meets a new runtime-fidelity instance, it applies these three
rules and files an entry. A question is for a case where they conflict.
