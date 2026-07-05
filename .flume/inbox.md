<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

## Wave-end confirmation FAILED — the dogfood gate silently checks nothing (2026-07-04, session)

The demolition wave is code-green (all cargo gates, all seven links shipped)
but the dogfood confirmation caught a live regression — the self-gate was off
for the wave, so this is the first post-wave `temper check .` on the repo's
own harness. Evidence, current binary (`cargo install` fresh at trunk head):

- `temper check .` reports **"checked 0 members across 4 built-in kinds"**
  and exits **0**. Before the wave the dogfood checked ~35 members.
- The lock (`.temper/lock.toml`) still carries the members: 2 `[[rule]]`,
  11 spec-kind rows, 2 memory-carrier rows, 24 `[[declaration.satisfies]]`
  joins, 2 `[[declaration.requirement]]` rows.
- The three registered spec kinds (intent/architecture/process) have vanished
  from coverage output entirely — presumably fallout of KIND-PACKAGE-PARSE-
  RETIRE compiling kinds to Rust data (the temper.toml `[kind.X]`
  registrations no longer reach check).
- Worse than the zero: **exit 0 despite two `required = true` requirements**
  (`engineering-standards`, `collaboration-discipline`) with zero members to
  fill them. 40-composition.md: absence blocks the gate. A required
  requirement over an empty member set must be a blocking finding, not
  silence. This is the exact silent-skip failure class temper exists to
  catch (cf. the qualified-kind checked-0 incident).
- Likely seam: `Workspace::load_kinds` (src/check.rs:63) still walks
  migration-era surface subdirs (`<dir>/<subdir>/*` member documents) while
  the wave moved the gate's other inputs onto committed artifacts + lock;
  whatever root `check .` passes it no longer contains what it expects.
  Not diagnosed further — plan's to scope.

Session holds: selfCheckGate stays OFF (re-arming against a broken check
would hard-block every future tick); the fence-side migration waits on a
working gate. `install.gate-installed` drift (session-start + new guard
hook) is session territory and waits behind the fix too.
