## Surface

The canonical path spelling now has one home (`src/path.rs`:
`to_canonical_spelling` / `canonical_spelling`), but one lock-named alias
survives beside it so the fold could land inside its fence:

- `src/drift.rs:42` — `pub(crate) use crate::path::to_canonical_spelling as
  to_lock_path;`, a compat re-export with exactly one consumer.
- `src/coverage_note.rs:159` — that consumer: `drift::to_lock_path(rel)`, the
  one caller of the moved function outside the
  `PATH-SEPARATOR-SPELLING-HAS-ONE-HOME` fence, so neither the call nor the
  alias could be retired this tick.

Two names for one function is the residue the fold exists to remove, and the
surviving one is the lock vocabulary the one home was renamed away from.

## Observed at

faa3fc7f (HEAD when observed) — the fold ships on top of it.

## Suggested consolidation

Re-point `src/coverage_note.rs:159` at `crate::path::to_canonical_spelling`
and delete `src/drift.rs:42`. One-line entry, fence `src/coverage_note.rs` +
`src/drift.rs`; no test moves (the spelling's pin already lives in
`src/path.rs`).
