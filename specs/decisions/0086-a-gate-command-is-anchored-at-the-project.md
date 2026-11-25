# 0086 — a gate command is anchored at the project

- **Date:** 2026-11-25 · **Status:** accepted

## Context

GH #67. 0082's rendered command resolves both its binary and its harness
target (`guard .`, `check . --reporter session-start`) against the working
directory. Claude Code runs a hook "in the current directory", and that
directory follows a Bash `cd`. It also exports `CLAUDE_PROJECT_DIR`, "the
project root where the session started", and recommends double-quoting it
in a shell-form command (code.claude.com/docs/en/hooks, retrieved
2026-11-25). So after one `cd`, a relative `binary` exits 127 and the
guard is down. An absolute binary still judges a subdirectory as the
harness root, and the bare `temper` default has the same target problem.
The hand-written `"$CLAUDE_PROJECT_DIR"/…` form that #65 replaced did not
fail this way. Ruled by the session under John's standing direction: act
where the solution is clear.

## Decision

**The engine anchors every gate command at the project root the provider
names** (`adoption.md`, "Install"). The command enters that directory
before its probe, so a relative `binary` and the `.` target both resolve
from the project root. Where the provider sets no root, the command runs
where it is invoked, as it does today. The binary stays the member's one
fact (0082). The anchor is the provider's, so it is rendered and never
authored.

## Rejected

- **Prefix only the binary and the target with the variable.** That puts
  the anchor in two places, and the bare-name default would still need a
  third spelling for a `.` target.
- **Let `binary` carry shell (`$CLAUDE_PROJECT_DIR/…`).** That makes a
  declared fact a shell fragment, and the installed-gate recognizer would
  have to parse it.
- **An absolute binary as the remedy.** It does not fix the target, and
  committing it carries a machine-local path (0082).

## Consequences

Every represented harness's gate commands change bytes on the next emit,
and the installed-gate check reads the old form as stale, naming the
re-emit as the remedy. The recognizer matches the anchored form only. The
migration note's relative-binary example now holds whatever the session's
directory.
