# 0082 — a gate hook names its binary

- **Date:** 2026-11-24 · **Status:** accepted

## Context

GH #65. `GUARD_COMMAND` calls a bare `temper` behind `command -v temper
|| exit 127`, and `install.gate-installed` compares a hook's command to it
by exact string (`install.rs`, `group_has_command`). An adopter with
temper installed in a workspace package, off the PATH, has two options,
and neither works:

- Keep the constant, and the guard exits 127 on every tool call.
- Call temper by path, and every session start reports the hook stale.

0073 gave the commands one home, the engine's. It did not give the binary
a declared place, so a binary's location could only be spelled as a
different string. Ruled by John 2026-11-24 on the session's
recommendation.

## Decision

**A gate hook states one fact, the temper binary it runs** (`adoption.md`,
"Install"), which defaults to `temper` on the PATH. The engine renders the
command from it, and the installed-gate check judges the typed member,
never a command string. The binary path is stated once (0078), and a
moved path is an edit to one value.

## Rejected

- **A harness-wide `temperBin` field.** It states the fact once for all
  three hooks, but it is a field of the root member about a property of
  three others. The binary is the hook's own fact, and the SDK constructor
  can default it for all three.
- **Install detects the binary** and writes its path into the command. A
  committed file would then carry a machine-local path, and the check
  would compare against whatever install guesses today.

## Consequences

0073's "the gate commands have one home" holds for the rendering; the
string constants stop being an author surface. The SDK gains a typed gate
hook constructor, and a hook member already scaffolded with the constant
upgrades through the migration note.
