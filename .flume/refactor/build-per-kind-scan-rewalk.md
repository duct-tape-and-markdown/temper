## Surface

`check`'s per-kind discovery re-walks the tree on top of the shared discovery
walk. `discoverable_paths` (`src/import.rs:505`) already walks the whole harness
once per flavor and returns every authored path as a `BTreeSet<PathBuf>`. Then
`scan_locus`/`collect_glob` (`src/import.rs:399`, `src/import.rs:429`) walk the
filesystem *again* — `read_dir` + `normalize_path` + `is_dir`/`is_file` per
entry — once per kind (and once per host unit for the nested-file kinds), only to
re-discover paths the shared set already holds. The `**` loci (`memory`'s
`**/CLAUDE.md` at root `.`, `agent`'s `**/*.md`) re-traverse the entire subtree
per kind.

Measured this tick on a generated 17,001-file harness (CHECK-RESIDUAL-DIAGNOSIS,
`tests/check_cost.rs`): after the glob-compilation memo cut discovery from
11,901 ms to 3,559 ms, the ~3.5s residual is this per-kind `read_dir` re-walk —
the read+hash phase over the same 15,301 members is only ~420 ms by comparison.

## Observed at

a4cffee (HEAD when observed).

## Suggested consolidation

Derive each kind's matches from the already-computed `discoverable` set instead
of re-walking the filesystem in `collect_glob`: the set is the authored path
universe, so a glob match over it replaces N per-kind directory walks with N
in-memory filters (the fork's "share the scan" half). The one fact `discoverable`
does not currently carry is file-vs-dir, which `collect_glob` reads via
`is_file`/`is_dir` — recording that at walk time (a set of paths tagged, or a
parallel dir set) lets the scan drop the filesystem entirely. Count-pin the
result the same way glob compilation now is: no directory read twice in a run.
