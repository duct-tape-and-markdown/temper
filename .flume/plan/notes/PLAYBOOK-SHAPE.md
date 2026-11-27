# PLAYBOOK-SHAPE: nested trigger landed; entry complete

`{ nested: "<dir>" }` joins `PlaybookTrigger`. The rule lands at
`<dir>/.claude/rules/<name>.md` and the skill at
`<dir>/.claude/skills/<name>/SKILL.md`. Both are named `<a>:<b>:<name>`
with declared locations. A nested rule carries no `paths`, since the
subdirectory load is the trigger. A non-plain dir (empty, absolute,
`.`/`..`/`.claude`, glob or `:` segment) is refused at construction.

Fix on the way: a nested skill's mention renders `/apps/web:x` with its
slash. The default user-only pointer hard-coded a leading `/` and printed
`//apps/web:x`. It now adds the slash only when the mention lacks one.

Probe: a throwaway harness via `defaultPatterns({ playbooks })` covered
paths, on-touch, nested (all three invocations) and none. It emitted and
`temper check . --deny-advisories` exited 0.

Adopter friction the probe hit: to reach exit 0, a minimal hand-built
`.temper/` needed `packageManager` in package.json, `tsc` in
`node_modules/.bin`, a lockfile, a declared `settings` member, and files
for each `paths` glob. Each refusal named its fix, so this is signal,
not debt.
