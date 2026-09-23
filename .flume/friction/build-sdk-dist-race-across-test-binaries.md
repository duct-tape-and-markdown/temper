## Symptom

`cargo test` reverted this entry's first attempt on a failure it had no hand in:
`tests/emit.rs::emit_program_runs_the_shipped_example_harness` died on
`ENOENT .../sdk/dist/src/prose.js`. The attempt's whole diff was one capture file
under `.flume/refactor/` — it touched no Rust at all.

`common::ensure_sdk_built` guards `npm run build` in `sdk/` with a `Once`, which is
per *test binary*, not per `cargo test` run. Cargo runs the test binaries in
parallel, so two of them (`tests/bundle.rs` and `tests/emit.rs` both appear in the
failing log, each with its own npm warning line) can enter the build concurrently —
one does `rm -rf dist && tsc` while the other is already reading `dist/`. The
reader sees a half-deleted tree. Nothing about the commit under test selects for
it; any tick can draw it.

## Cost this tick

One reverted commit and one full re-dispatch of `CONTAINMENT-INCIDENCE-FAMILY`
(~an entire tick), plus the second attempt re-deriving the diagnosis from the gate
log. The entry's own work was never implicated.

## Suggested fix

Take the build out of the test binaries' hands: a cross-process lock (a lockfile
in `sdk/`, or `tsc --outDir` into a per-binary temp dir), or a `build.rs`/`Makefile`
step that builds `sdk/dist` once before `cargo test` forks. `tests/common/mod.rs`
already documents the intra-binary race it solved (`vendor_sdk`'s "joins that
`Once` instead of racing a sibling test's `rm -rf dist && tsc`") — this is the same
race one scope out.
