# Anchor every rendered gate command at the project root (GH #67, Decision 0086)

Urgent: after any Bash `cd`, the guard is down. `render_gate_command` (`src/install.rs`) emits `command -v {word} … && {word} guard .`, and Claude Code runs hooks in the session's current directory, which follows `cd`. A relative `binary` exits 127, and an absolute one judges a subdirectory as the harness root. The bare `temper` default has the target problem too, this repo's own hooks included.

The ruling (`specs/model/adoption.md`, "Install"; 0086): the command enters the provider's project root before its probe, so the binary and the `.` target resolve there. The spelling the docs recommend for shell form is the double-quoted variable (code.claude.com/docs/en/hooks, retrieved 2026-11-25). Where the variable is unset, the command runs where it is invoked, for example `cd "${CLAUDE_PROJECT_DIR:-.}" && command -v … && … guard .`. `rendered_gate_binary` recognizes the anchored form only, and the renderer stays the one home (0073). The census includes:

- the `SESSION_START_COMMAND`/`GUARD_COMMAND` constants and every pin of their bytes;
- the base-harness example lock;
- the dogfood `.temper` lock and `.claude/settings.json` (a human `chore(harness)` re-emit after the ship).

Acceptance: from a subdirectory with `CLAUDE_PROJECT_DIR` set to the root, each rendered gate command, run through `sh -c`, invokes temper at the root with the relative binary. With the variable unset, the command behaves as before.
