## Symptom

`examples/base-harness/.temper/lock.toml` is a **derived** artifact of every
built-in kind fact — the example composes two `hook` members, so its emitted lock
carries the `hook` kind row and every column a kind-fact change moves — and
`tests/it/emit.rs`'s `emit_program_runs_the_shipped_example_harness` byte-compares
the committed file against a live emit. It is the third member of the set
`src/builtin_lock.toml` and `tests/it/snapshots/**` already belong to: never
hand-authored, so a diff there is either the regeneration or a red test.

The other two are fence-standing. `tests/**/snapshots/**` sits in
`.flume/chain.ts`'s `BUILD_SURFACE_PATHS` (since `b7b55457`, the fix drained out
of the sibling capture in this directory); `src/builtin_lock.toml` is named by
every entry that moves it. The example lock is named by *some* entries and
missed by others, and a miss is invisible until the fence reverts an otherwise
green commit.

## Cost this tick

Two build ticks in a row filed a re-scope capture instead of a commit, both over
this file class:

- `1ad4fda1` (EMPTY-CONTRACT-IS-A-DECLARED-CONTRACT): three of the four
  regeneration commands its capture listed were this one file.
- `dbef8c68` (HANDLER-IS-A-BUILT-IN-KIND-UNDER-HOOK): **one** out-of-fence path,
  this file. Four of five gates verified green, `cargo test --no-fail-fast` down
  to exactly one failure, and nothing but the capture was committed —
  99 turns / 14.9 min / 62k output tokens re-bought on the retry.

Plan then spends a tick per occurrence re-scoping, and the retry re-runs work
already proven green.

## Suggested fix

One glob added to `.flume/chain.ts`'s `BUILD_SURFACE_PATHS`, beside the snapshot
and seam-binding entries it matches in kind:

```
  "sdk/src/generated/**",
+ // The example harness's own emitted lock: derived from every built-in kind
+ // fact, byte-compared by tests/it/emit.rs, regenerated (never authored) by
+ // whichever build moves a fact it carries. Two consecutive fence-short ticks
+ // (1ad4fda1, dbef8c68) were this file.
+ "examples/*/.temper/lock.toml",
```

Narrow on purpose: `examples/**` is already inside `BUILD_WRITABLE_PATHS`, so
this only lifts the per-entry `files[]` requirement for the derived lock, not for
the example's authored modules beside it. Human territory (`chore(flume):`) — the
chain's fence config is outside the amendments channel's in-scope targets (0044),
the same routing the sibling snapshot-glob capture took.
