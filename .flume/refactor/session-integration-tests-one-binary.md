## Surface

The integration suite is 57 test crates: every `tests/*.rs` compiles as its
own crate and links its own executable, each pulling in the whole `temper`
lib and its dependencies. 53 of the 57 declare `mod common;`, so
`tests/common/mod.rs` (782 lines, one home in source) compiles and links 53
times over. On disk, 181 debug executables in `target/debug/deps/` are over
30 MB each (`graph-*` is ~71 MB).

Cost paid this session, from the kernel log of the 14:42 OOM that rebooted
the 7.7 GB WSL VM: at the kill, 17 `rust-lld` and 20 `rustc` processes held
2.6 GB. That was the `cargo test` gate linking these binaries at cargo's
default job count, beside flume's own loop. The same pattern hit at 14:35.
The loop now runs with `CARGO_BUILD_JOBS=4` (ledger) and the VM has 11 GB,
which treats the symptom. The judge the harness migration adds (a nextest
run in the worktree plus a base build per judged entry) pays this link
cost twice per entry.

Cargo's own guidance: "Each integration test results in a separate
executable binary … If you have a lot of integration tests, you may want to
consider creating a single integration test, and split the tests into
multiple modules" (doc.rust-lang.org/cargo/reference/cargo-targets.html,
"Integration tests", retrieved 2026-09-22).

## Observed at

4dd4583f

## Suggested consolidation

One integration crate: `tests/it/main.rs` declaring each current
`tests/<name>.rs` as `mod <name>;` (moved to `tests/it/<name>.rs`), with
`common` declared once, plus `[[test]] name = "it"` if autodiscovery needs
it. Snapshot paths under `tests/snapshots/` key on the module path, so a
move re-blesses them (`insta`). Plan should measure the win first
(`specs/process/engineering.md`, "Diagnosis is measure-first"): link time
and peak RSS of `cargo test --no-run` before and after. Two ordering notes:
the move renames every nextest binary-id the judge's reader resolves
against (from `temper::graph$…` to `temper::it$graph::…`), so land it before
the harness migration or with it, never after; and it touches every tests/
file, so serialize it behind the in-flight entries that edit tests/.
