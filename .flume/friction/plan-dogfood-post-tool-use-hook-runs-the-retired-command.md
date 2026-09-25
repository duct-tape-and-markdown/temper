## Symptom

This repo's own `PostToolUse` gate is dead, and nothing says so.

`.temper/hooks.ts:42` (`hook_postToolUseBash`) runs
`temper check . --reporter session-start`. That reporter stamps
`hookEventName: "SessionStart"`, and Claude Code accepts only the firing
event's name — so the row is rejected whole on every Bash call. 5820ad7b fixed
exactly this in the product (`GATE_HOOKS`' `PostToolUse` row now runs
`temper guard .`, which reads the payload's own event) and named the mirror as
a follow-up outside build's fence. Verified on disk this tick: the module, and
its `.claude/settings.json` projection at `PostToolUse[0]`, still carry the old
command. The module's doc-comment also cites `POST_TOOL_USE_COMMAND`, a
constant that commit retired.

`check` cannot catch it by design: the event *is* claimed by an authored `hook`
member, so `gate_outcome` answers `SupersededByMember`, and `gate_installed`
skips that outcome (an author who owns the event may run their own command).
`cargo run -- check .` this tick reported `coverage.checked` alone.

Two smaller restatements went stale with it, both in `docs/` (human territory):
`docs/cli.md:69` still opens the `guard` section "A `PreToolUse` hook body",
and `docs/market-formats.md:143`'s `paths` probe record says "until Claude
reads a file", the wording 0061 narrowed to a file *tool*.

## Cost this tick

No revert — the audit motion found it and the entry work was unaffected. The
real cost is standing: every Bash-mediated projection edit in this repo has
gone unguarded since the row was wired, and the next shipped guard change
inherits the same silent mirror.

## Suggested fix

One line, apply-ready: `.temper/hooks.ts:42`'s command becomes
`` `${failLoud} temper guard .` `` (byte-identical to `GUARD_COMMAND`, as the
`PreToolUse` member above it already is), the doc-comment at :33 stops naming
`POST_TOOL_USE_COMMAND` and says one command serves both edges, then
`temper emit`. Docs: `docs/cli.md:69` → the guard runs at both edges;
`docs/market-formats.md:143` → "reads a matching file with a file tool".

The durable half is the silence: a hand-mirrored gate command has no gate.
Either the mirror stops being by hand (the SDK exports the commands the way
`TAP_COMMAND` already is — see the hooks.ts note on why the tap needs no
mirror), or `SupersededByMember` says something when the member's command is a
*stale spelling* of temper's own rather than an author's deliberate one.
