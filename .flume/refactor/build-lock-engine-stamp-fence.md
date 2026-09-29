# build-lock-engine-stamp-fence

`LOCK-NAMES-THE-ENGINE-THAT-WROTE-IT` cannot reach green inside its fence.

## Surface

The entry's premise — spell the stamp outside `[declaration]` because "the
frozen lane byte-compares exactly those rows" — does not hold. The frozen lane
compares the memberless emit's **whole lock text** against the embedded file
sliced from `[[declaration.kind]]`:

- `tests/it/builtin_lock_frozen.rs:99-108` — `embedded_declaration_rows()`
  slices `src/builtin_lock.toml` from `[[declaration.kind]]` to EOF.
- `tests/it/builtin_lock_frozen.rs:118-130` — `derived` is the emitted
  `lock.toml` read whole, asserted equal to that slice.

It passes today only because a memberless emit writes nothing ahead of the
declaration rows. A root-level TOML key must precede every table, so an
unconditional `engine = "<version>"` lands as a prefix on the derived side and
the compare fails on that prefix alone:

    left:  "engine = \"0.0.20\"\n\n[[declaration.kind]]\nname = \"agent\"\n…"
    right: "[[declaration.kind]]\nname = \"agent\"\n…"

Moving the stamp to a trailing `[engine]` table only turns the prefix into a
suffix; moving it inside `[declaration]` is what the entry rules out, and is the
one region the compare holds byte-exact anyway. There is no spelling that keeps
the frozen lane green without editing it.

Needed, outside the fence (inside the outer ceiling):

- `tests/it/builtin_lock_frozen.rs` — slice the derived side from
  `[[declaration.kind]]` too, so both sides are the declaration rows the compare
  claims to hold. One shared helper over the two texts.

**Not** the fix: stamping `src/builtin_lock.toml`. The embedded lock is a frozen
artifact re-derived by that very compare; a live version line in it breaks on
every release bump.

## What the fence did cover

The other three files reach green as scoped, and the remaining red target under
`cargo test --no-fail-fast` is the one above:

- `src/drift.rs` — `ENGINE_KEY` written by `write_rollup` at the document root;
  `engine_from_doc` lifts it as `Option<String>` (absent key, and a present
  non-string, both read unknown — never a `LockRowError`); `read_engine` is the
  counted door onto it. `write_rollup`'s stale `read_declarations` cite
  corrected to `declarations_from_doc`.
- `tests/it/emit.rs` — the round-trip pin, plus the shipped-example lock compare
  normalized: the stamp is dropped from both sides, since committing a version
  line into `examples/base-harness/.temper/lock.toml` would churn a shipped
  artifact on every release bump, and that lock's absent key is the same
  legitimate absence the dogfood's is. If plan would rather the example's
  committed lock carry the key, `examples/base-harness/.temper/lock.toml` needs
  the fence too and the compare normalizes the version instead of dropping it.
- `tests/it/gauntlet.rs` + its `projection_tree` snapshot — an insta filter
  rewrites the rendered lock's `engine` line to `engine = "<VERSION>"`, pinning
  the key's presence rather than the release.

`cargo fmt --all --check`, `cargo clippy --all-targets -D warnings` and
`cargo doc` are clean over that work.

## Adjacent, not taken

The lock's root namespace mixes reserved keys with author-chosen kind names, so
a kind named `engine` has its rollup array overwrite the stamp (degrading the
read to unknown) — and a kind named `declaration` already loses its rollup to
`Declarations::write_into` the same way. Pre-existing shape question; the stamp
joins it rather than creating it.

## Observed at

76eac90a

## Suggested re-scope

Add `tests/it/builtin_lock_frozen.rs` to the entry's `files.edit` with the
derived-side slice named, and re-dispatch. Nothing else moves.
