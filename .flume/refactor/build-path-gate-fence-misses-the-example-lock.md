## Surface

`PATH-GATE-OPENS-ON-FILE-TOOLS` is scoped to three files
(`sdk/src/builtins.ts`, `sdk/src/contract.ts`, `src/builtin_lock.toml`), but
a fourth checked-in lock carries the same shipped guidance rows and is
byte-compared by a test outside the fence:

- `examples/base-harness/.temper/lock.toml:299` — `skill.glob-valid.paths`,
  guidance `"…gates every invocation channel until Claude reads a file its
  globs match…"`.
- `examples/base-harness/.temper/lock.toml:308` — `skill.mention-reachable.paths`,
  guidance `"…until Claude reads a matching file…"` plus the `cite` that gains
  the 2.1.281 probe.

`tests/emit.rs:2480`
(`emit_program_runs_the_shipped_example_harness`) re-emits the example's own
program and byte-compares it against that checked-in lock, so moving the two
SDK guidance strings turns the test red with no in-fence file that can close
it. Verified under `cargo test --no-fail-fast`: it is the only red target —
all other 40+ test binaries stay green with the SDK edits and a regenerated
`src/builtin_lock.toml` applied.

Regeneration is not a hand edit either: the example's lock comes from
`cargo run -- emit --into examples/base-harness/.temper`, which needs
`@dtmd/temper` vendored into `examples/base-harness/.temper/node_modules`
the way `tests/common/mod.rs::wire_sdk_harness` does it.

Adjacent, outside every fence and not obviously in scope: `docs/market-formats.md:143`
carries the same retired wording, but as a dated field-research note
(local probes, 2.1.210, 2026-07-15) rather than shipped guidance — plan's call
whether a research note gets restated or left as the record of what was probed then.

## Observed at

6bfc0893 (HEAD when observed) — plan diffs forward from here.

## Suggested consolidation

Re-scope the entry to add `examples/base-harness/.temper/lock.toml` to its
`files.edit`. More durably: any entry that moves a shipped default-contract
guidance string touches *two* derived locks, and only one of them
(`src/builtin_lock.toml`) has a name that says so. Either fold the example's
lock into whatever scoping rule already pairs a `sdk/src/builtins.ts` edit
with `src/builtin_lock.toml`, or give the repo one regeneration entry point
(a `just`/`cargo xtask` target that vendors the SDK and re-emits both locks)
so the pairing is mechanical rather than remembered per entry.
