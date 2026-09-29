## Surface

Two homes write a test lock, and the hand-spelling one is being reached for
where the real writer would do:

- `tests/it/common/mod.rs:943` `common::write_lock(root, Declarations)` — the
  real writer (`drift::emit`), ~40 call sites (`requirement_roster.rs`,
  `root_contract.rs`, `layout_kind.rs`, `dial_kind.rs`, …). For a member-less
  payload it writes exactly one file, `.temper/lock.toml`, and stamps the
  clause `label`s itself.
- `tests/it/common/mod.rs:459` `GuardLock` — a hand renderer, warranted: a
  guard fixture's whole point is a row at a fingerprint no file's bytes can
  produce.
- `tests/it/acceptance.rs:150` — a *third* home: a private
  `write_lock(corpus, &str)` writing raw TOML, shadowing the name of the
  first.

Measured while executing `SESSION-START-LOCK-FIXTURES-FOLD-INTO-ONE-HOME`:
`common::write_lock` reproduces the richest of that entry's four hand-spelled
locks **byte-for-byte**, `label = "spec.extent"` included. None of the four
declared a member row, so none needed a hand renderer at all.

The class remainder that entry filed for later is the same shape — plain
declaration rows the real writer produces, not drifted ones:
`acceptance.rs` (6, via the private home above), `check_cost.rs` (3),
`hook_kind.rs:498` (kind rows with `collection_address`), `cli.rs:263` (the
exact requirement row this tick converted), `coverage_note.rs:485`.
`coverage_note.rs:292` is the real exception: a deliberately corrupt lock, a
refusal input no writer can emit.

## Observed at

4365901c (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation

Keep both homes, but pick between them by what the fixture needs rather than
by which suite it sits in: `common::write_lock` for any lock whose rows the
real writer can produce, `LockFixture`/`GuardLock` only for drifted
fingerprints and malformed bytes. Concretely — retire
`tests/it/acceptance.rs:150`'s private raw-TOML home onto
`common::write_lock`, and re-scope the remainder above the same way rather
than onto a widened hand renderer.
