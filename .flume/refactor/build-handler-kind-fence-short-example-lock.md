## Surface

`HANDLER-IS-A-BUILT-IN-KIND-UNDER-HOOK` is fence-short by **one** path. The
entry's design is right and reaches green — implemented whole this tick, four of
five gates verified (`cargo fmt --all --check` clean, `cargo clippy --all-targets
-D warnings -D clippy::todo -D clippy::unimplemented` clean, `pnpm --dir sdk test`
216 pass, `cargo doc` exit 0) and `cargo test --no-fail-fast` down to exactly one
failure, which is that path. Nothing but this capture was committed.

**The out-of-fence path — `examples/base-harness/.temper/lock.toml`.** The example
harness composes two `hook` members, so its own emitted lock carries the `hook`
kind row and gains the same seven derived lines `src/builtin_lock.toml` does: a
`handler` kind-fact row, and `templates = [{ kind = "handler" }]` on `hook`'s row.
`tests/it/emit.rs`'s `emit_program_runs_the_shipped_example_harness` byte-compares
the committed lock against a live emit, so it is the one red target. No hand edit —
regenerate:

```
ln -sfn $PWD/sdk examples/base-harness/.temper/node_modules/@dtmd/temper   # gitignored
cargo run -- emit --into examples/base-harness/.temper
```

This is the identical fence gap `build-empty-contract-fence-short-six-paths.md`
hit at `cc46bbb1`: **any entry that moves a kind fact of a kind the example
harness composes moves the example's lock too.** Three of the four `.flume/`
gitignore-clean regeneration commands in that capture were about the same file.

Everything else the entry named was sufficient and green: `sdk/src/builtins.ts`
(`Handler`, `handler`, `handlerDefaultContract`, `hook.templates`, the spread),
`sdk/src/claude-code.ts`, `sdk/test/builtins.test.ts`, `src/builtin_kind.rs`
(`claude_code_handler()`, `hook`'s template entry, the definition unit test),
`src/builtin_lock.rs`, `src/builtin_lock.toml` (regenerated, header counts moved),
`tests/it/builtin_lock_frozen.rs`, `tests/it/lock_declaration_rows.rs`, and all
three declared snapshots.

Three things the next attempt should carry that the entry did not name:

1. **`handler` needs a fourth fact to earn its kind-fact row.** The entry
   specified `locus`/`registration`/`shape` only, but
   `sdk/src/declarations.ts`'s `kindFactKindsInPlay` gives an *embedded* kind a
   row only where it carries `guidance`, `cite` or `leaves` — otherwise its
   members reach the corpus through the host's `templates` column alone. With
   those three facts the lock got the six clause rows and `hook`'s templates
   column but **no `handler` kind row**, which then fails
   `src/builtin_lock.rs`'s name-set compare against `builtin_kind::definitions()`.
   Resolved by giving the kind the `cite` that `specs/builtins.md`, "The shipped
   kinds" already demands of it (`https://code.claude.com/docs/en/hooks
   (retrieved 2026-09-29)` — the `hooks`-array and positional-key fact), so the
   row is earned rather than manufactured. Worth stating in the re-scope.

2. **`tests/it/snapshots/it__gauntlet__check_diagnostics.snap` does move.** The
   entry predicted no diff ("a memberless embedded kind adds no row"). The
   coverage line's kind set is built from the *declared* kinds, not member
   counts, so it goes `22 kinds` → `23 kinds` with `handler (0)` joining. Both
   gauntlet snapshots and the contract matrix re-bless with
   `INSTA_UPDATE=always`; all three were in the fence, so this cost nothing —
   recorded only so the next scoping does not read the diff as a surprise.

3. **The locus-spelling choice the fence could not fund.** The entry offered
   "either `CustomKind::nested_file` is renamed to the locus it actually spells
   or an embedded sibling joins it — name the choice in the commit body", but
   `src/kind.rs` is outside the fence and `with_locus` is private, so neither
   option was reachable. Taken: **reuse `nested_file` as-is**, with the call site
   naming that the constructor is named for the file case rather than for the
   `None` governs it writes. The same gap exists on the SDK side — `KindFacts`'s
   embedded variant makes `unitShape` *required*, so `handler` spells `file` for
   a member that owns no unit, which is what every embedded kind in the suite
   already does. Both are one gap: the two locus spellings each carry a fact that
   the embedded case has no value for.

## Observed at

a5387d50 (HEAD when observed) — the premise delta since the entry was scoped at
2d9f5088 (`32c3497b`, the declared-contract markers) changed none of the above
beyond adding a nineteenth assembly fact row for `handler`, which the
regeneration picked up.

## Suggested consolidation

Re-scope the entry with `examples/base-harness/.temper/lock.toml` added to
`entry.files.edit`, described as the derived example lock the `hook` kind fact
moves, with the regeneration command above. Fold in points 1 and 3: the `cite`
belongs in the kind's file description, and the locus-spelling choice wants
`src/kind.rs` + `sdk/src/kind.ts` in the fence if the rename is to land with the
kind rather than after it.

The standing shape is worth a durable fix rather than a per-entry patch: while
`examples/base-harness/` composes built-in members, its lock is a derived
artifact of *every* built-in kind fact, exactly as `src/builtin_lock.toml` and
`it__gauntlet__projection_tree.snap` are. Those two are already fence-standing
(`tests/**/snapshots/**` is a blanket allow after the glob fix in
`build-snapshot-fence-glob-misses-tests-it-snapshots.md`); the example lock is
the third member of that set and is not. Adding
`examples/*/.temper/lock.toml` to the build surface's standing paths would
retire this class of fence-short tick, and it is safe on the same grounds the
snapshot blanket is: the file is never hand-authored, so a diff there is either
the regeneration or a red test.
