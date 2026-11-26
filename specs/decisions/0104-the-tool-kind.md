# 0104 — the tool kind

- **Date:** 2026-11-25 · **Status:** accepted

## Context

0100 made a command one typed reference with runner edges, but it named no
home for it (`command-reference-declaration`). Edges run between members,
and `command` is already the legacy skill kind's name. The question also
asked which verb reports an unrun command, which lockfiles are read, and
what the default is. `settings-referenced-scripts` waits on the same answer.
This is the "one concept, one member kind" class, after 0078.

## Decision

**`tool` is a shipped embedded kind, the one home of a command**
(`builtins.md`, "Tool"):

- It registers nothing. Runners (gate hooks, taps, the typecheck, script
  verifiers, a hook running a script) reach it through edges.
- Its default contract is one advisory `degree` clause, incoming at least
  one, so `check` reports a tool nothing runs. It uses an existing
  predicate, so it needs no new finding kind.
- A `{ package, bin }` tool is checked against `package-lock.json`, and
  against `pnpm-lock.yaml` where a sanctioned crate's YAML reader covers
  it. Any other lockfile refuses loudly. The version shows in `explain` and
  in the reference, and a package missing from the lock is a finding.
- The default is the temper package's bin, rendered as the anchored `.bin`
  path. An unrepresented harness keeps `temper` on the PATH.
- A settings-referenced script is a `{ file }` tool that its hook or status
  line runs. An unrepresented harness reports such scripts as one grouped
  coverage note (0103).

## Rejected

- **A harness-level declaration with its own row family.** Edges need
  members, and this would be a second mechanism.

## Consequences

0098's typecheck and the three `binary` fields lower onto `tool` members.
