## Surface

`SCAN-SHARE-DISCOVERABLE-SET` moved discovery off the filesystem: the per-kind
glob scan now reads the shared walk's in-memory index and the walk swallows I/O
errors via `walk.flatten()` (`src/import.rs:551`). That leaves discovery
infallible at the read level, so:

- `ImportError::ReadDir` (`src/import.rs:122`) is now **unconstructable** — its
  only builders were `read_entries`, retired this tick. `ImportError::Write`
  (the roll-up writer) is the only live variant.
- The whole `discover_*` chain still returns `Result<_, ImportError>` while no
  arm can `Err`: `scan_locus` (399), `discover_kind_units` (375),
  `discover_kind_files` (340), `discover_nested_file` (234),
  `declared_governed_paths` (286), `discover_builtin` (188) — vacuous `?`
  plumbing whose callers (`src/main.rs:1353,1372`, `src/json_manifest.rs:431`,
  `src/install.rs:332`) all `?` a value that never fails.

## Observed at

8b06146 (HEAD when observed); the cut lands one commit later on
`flume/scan-share-discoverable-set`.

## Suggested consolidation

Drop `ImportError::ReadDir` and make the `discover_*` fns infallible (return the
bare `Vec`/`BTreeSet`), leaving `ImportError` to the roll-up writer alone — or
rename it to a writer-scoped error. Out of scope this tick: collapsing the
`Result` signatures ripples through `main.rs`/`json_manifest.rs`/`install.rs`
and their tests, well past the re-walk cut.
