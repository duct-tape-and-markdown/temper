## Surface

A local-locus member's derived rows reach one of the two corpora that read
them, and a false-green test hides the half that misses.

- `src/main.rs:1555` `embedded_features_by_kind` folds `declarations.nested_members`
  — the committed lock's rows — and nothing else. Both call sites
  (`src/main.rs:523` read arm, `src/main.rs:913` `gate`) pass only
  `&declarations`.
- `src/main.rs:1402` `local_document_rows` derives a local member's rows at read
  time, but its `nested` half lands only in the *host's* own
  `Features::nested_members` (`src/main.rs:1384`). So a clause bound to the
  embedded kind selects zero members for a local host, while the same clause over
  a committed host selects normally. `satisfies` has no such gap — it folds at
  `src/main.rs:1229` and reaches the roster.
- `tests/local_locus.rs:284` `check_derives_a_local_members_collection_members_off_its_document`
  is the test that should pin this, and pins nothing: its clause row declares
  severity `"error"`, outside the closed `required`/`advisory` vocabulary, so the
  lock refuses at load and the test's `!ok` passes on the load refusal. Give it a
  valid severity and it fails — the count clause never fires.

This is the read-side half of the same fact `pipeline.md`, "The lock" states as
the **lock family**: the committed lock plus the read-time rows of local-locus
members. One assembled family would also retire the commitment branches at
`src/main.rs:1229` and `src/main.rs:1376`, which exist only because the two
sources are never joined.

## Observed at

732dbcd (HEAD when observed) — surfaced by LOCAL-LOCUS-IS-READ-SIDE-ONLY, which
retires the layout-only fence and so widens the set of kinds that reach
`local_document_rows`. The gap predates that entry and is orthogonal to format:
it hits a layout local host identically.

## Suggested consolidation

Assemble the lock family once per run — committed rows plus every local kind's
derived rows — and let `embedded_features_by_kind` and the two kind loops read
that one `Declarations`, dropping their per-call-site commitment branches. Fix
the test's severity in the same pass so it pins the derivation rather than a load
refusal. Likely the same surface `CHECK-ANNOUNCES-THE-LOCK-FAMILY` already wants.
