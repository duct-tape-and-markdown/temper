<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- DEFECT (from friction, session-verified premise; RULED under the 07-18 delegation):
  `temper emit --into <workspace>` run from a cwd other than the workspace root rewrites
  every lock `source_path` row cwd-relative (20 corrupted rows beside 2 intended at
  examples/base-harness), exit 0, only tell an orphan-drift line. Ruling: source paths
  resolve against the --into root, never cwd — the flag names the workspace, and lock
  content varying by invocation directory violates emit's pure-function promise
  (0034/byte-faithful determinism). Refusing a non-root cwd is the acceptable fallback
  only if root-resolution proves unreachable. Fix ships the test that would have caught
  it: an emit --into from a foreign cwd asserting byte-identical lock vs a root-cwd run.
  observed at 88da37f

- NOTE (friction drained 07-18, for the record): the clause-algebra capture
  (nested-field addressing) was verified largely discharged at HEAD — FieldPath/locate
  shipped, marketplaceDefaultContract adopted the path clauses; sole live residue is the
  discriminated-union predicate, put to John directly (not an entry, not a fork yet).
  The hop-cap capture duplicated parked IMPORT-HOP-CAP-CITE — dropped as tracked.
  observed at 88da37f
