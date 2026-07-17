## Surface

Six integration suites each carry a **byte-identical** private copy of
`fn check_harness(harness: &Path) -> (Vec<String>, bool)` — run `temper check
--harness <dir> --reporter github`, keep the `::`-prefixed lines, pair them with
the exit status:

- `tests/coverage_note.rs:99`
- `tests/hook_kind.rs:64`
- `tests/installed_plugin_kind.rs:61`
- `tests/marketplace_kind.rs:64`
- `tests/mcp_server_kind.rs:59`
- `tests/plugin_manifest_kind.rs:55`

All six bodies hash identically. Each is a thin wrapper over the shared
`common::check_in`, so the shared home already exists — the wrapper is what got
copy-pasted, six times. `tests/memory_gate.rs:59` is a near-duplicate of the same
job differing only in dropping the exit status (`-> Vec<String>`).

`specs/process/engineering.md` ("One job, one home") names this case in as many
words: "Test scaffolding is a surface too: shared fixtures and builders live in
one home (`tests/common`), never copy-pasted per file."

## Observed at

2e27bdc (HEAD when observed) — read while scoping BUNDLE-EMIT-THROUGH-KINDS.

## Suggested consolidation

Lift one `check_harness` into `tests/common` beside `check_in` and delete the six
copies; fold `memory_gate.rs`'s variant in by letting the caller ignore the exit
status. Not taken this tick: BUNDLE-EMIT-THROUGH-KINDS is `src/bundle.rs`'s
manifest writers, and a seven-suite sweep is the creep that page forbids — the
gate test added to `tests/bundle.rs` calls `common::check_in` directly rather than
adding a seventh copy.
