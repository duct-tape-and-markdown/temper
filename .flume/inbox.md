<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->







## The settings member has no typed channel for residue: an undocumented key renders but fails tsc — observed at 18079383

Dogfooding 0050 (this repo's `.temper/harness.ts`, commit 18079383): the committed
settings file carries `worktree: { bgIsolation: "none" }`, which the settings
reference does not document (code.claude.com/docs/en/settings-reference,
retrieved 2026-09-22). Authored on `settings({...})`, it **renders**: emit
writes `.claude/settings.json` byte-identical, so the runtime passes residue
through. But strict `tsc` refuses it: `TS2353: Object literal may only
specify known properties, and 'worktree' does not exist in type
'MemberInit<Settings>'`. The `Settings` interface (sdk/src/builtins.ts,
`export interface Settings`) types the documented keys and has no channel for
the rest. 0050 decides the posture: "documented keys typed, residue opaque and
named". So the type owes a named, opaque residue channel that still refuses a
misspelled documented key. An index signature would swallow typos, so the
likely shape is a named bag. It is observed on this repo only because no gate
typechecks `.temper/`. An adopter's editor shows the red squiggle immediately.
The same question applies to `SettingsLocal`.
