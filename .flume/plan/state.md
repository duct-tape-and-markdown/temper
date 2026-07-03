# Plan state

- **Phase:** reconcile. HEAD af54371.
- **Last shipped:** IMPACT-VERB + INSTALL-GUARD-ARTIFACTS (chore af54371) —
  the `impact` read verb (src/read.rs) and the surface-authority guard
  artifacts wired into `install` (src/install.rs). Both dropped from the queue.
- **This tick:** drained the inbox's DIRECTIVE-PATH-NORMALIZE note by
  **reproducing it on disk** (`temper check .temper` fires a false
  `graph.directive-unbacked` on CLAUDE.md → @docs/ledger.md) and **challenging
  its root cause**. The inbox blamed unnormalized `././` provenance, but
  `graph::normalize_path` already collapses both sides symmetrically (verified:
  `check --harness .`, parent `.`, is clean). Real cause: the two-step `check`
  path passes bare `Path::new("temper.toml")`, whose `.parent()` is `Some("")`
  (not `None`), so `base_dir=""` and `repo_file_set("")` walks nothing — an
  empty world file-set forges an unbacked finding on every real `@import` (law
  3, missing file → false positive). Filed as **DIRECTIVE-BACKING-BASE-DIR**
  (open) with the correct fix in `gate`'s base_dir line. Re-verified the 5
  carried parked/deferred entries against disk — all still accurate (no
  `ignore` crate; `Field` still flat / no `Fenced` at kind.rs; BUILTIN_KINDS =
  `["skill","rule"]`; package.json still `temper-flume-harness`/private; no
  CONTRIBUTING/SECURITY).
- **Pickable now:** DIRECTIVE-BACKING-BASE-DIR (src/main.rs + tests/memory_gate.rs)
  — the sole open entry, no shared-file contention. WALK-IGNORE-DISCIPLINE and
  the 4 deferred/parked entries stay human-gated.
- **Accepted debt (not filed):** the `././CLAUDE.md` source_path provenance is
  written unnormalized (lock.toml / .temper/CLAUDE/MEMORY.md); harmless given
  the symmetric compare-side normalize, no consumer needs it normalized.
- **Operational note (accepted, not queued):** the session-start
  `requirement.dangling` findings are a **stale installed binary** — a freshly
  built `./target/debug/temper check .temper` no longer shows them (`cargo
  install --path .` clears the stale global).

Plan continues: no — inbox drained, one disjoint open entry pickable; hand to build.
