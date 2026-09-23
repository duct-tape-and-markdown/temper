<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->






## EXAMPLE-EMIT-GATE-BUILDS-THE-SDK-IT-READS's revert was checkout residue, now cleared — observed at 4e03d810

The entry's own assertion ("nothing may create
examples/base-harness/.temper/node_modules in the checkout",
tests/emit.rs:2270) fired on trunk because the *old* test left that
gitignored directory behind at 15:55: a lone `@dtmd/temper -> sdk`
symlink in the primary checkout, never tracked. The worktree had none, so the
entry was green there. The residue is removed. The entry's code is right as
built. Re-queue it unchanged, no re-scope. It was quarantined for this run
only.
