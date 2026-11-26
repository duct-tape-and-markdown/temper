# AGENTS.md two gates: scope read off position, no row column

The two gates shipped without a new `AbsenceGateRow` column. Which gate applies
is the document's position: `absence_closer` (src/kind.rs) judges it from its
home directory, with a `.claude/` folder folded into the directory that holds
it. A document homed at the harness root gets the launch gate. One homed below
the root gets the on-demand gate, checked against its own directory only. Within
the harness, "the working directory or above" for a root document is just the
root, so a declared scope would have been a second spelling of a fact the path
already carries. So `drift.rs`, `sdk/src/declarations.ts`, the generated
binding and the lock's memory row did not move. `GatedTemplate`'s doc
(sdk/src/kind.ts) states the two-gate semantics instead of a field. This
applies to every absence gate, a custom kind's included.

`.claude/AGENTS.md` was already a member: `**/AGENTS.md` matched it, named
`.claude:AGENTS`. Before this change, a root `CLAUDE.md` shut it only by
accident, through the old at-or-above walk. Now it shares the root's home.

The reachability reason now names the gate that shut ("its launch gate is shut:
…" / "its on-demand gate is shut: … is its own subdirectory's"). The three
tests[] lines assert that wording, which is what fails on the base. On the base
the latter two already drew a finding, just with the old reason.

Retired the old test that had a `CLAUDE.local.md` above a nested `AGENTS.md`
shadow it; 0113 rejects that case.

Re-emitting examples/base-harness needed typescript, `@types/node` and the SDK
symlinked into its `.temper/node_modules`. The test's remedy text names only the
SDK link.
