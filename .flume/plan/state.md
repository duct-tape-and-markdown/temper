# Plan state

- Spec derived through: 8911c38 — unchanged, not this tick's job.
- Audited through: a4926ca — window bfe147c..a4926ca (feef754, 5009090, d3cb30c, b5f7d7c) audited: all three shipped fixes (READ-DIAL-OVERLAY-MIGRATE, GUARD-MEMBERSHIP-FALLTHROUGH-RESTORE, TAP-HOOK-DEDUP-KEY-SEPARATOR-RESTORE) re-verified on disk against the shipped diffs and proven green (`cargo test`, `cargo clippy --all-targets -- -D warnings`, `cargo fmt --all --check`, `pnpm --dir sdk test` all pass); pending.json already reflects them drained.
- Residue swept through: a4926ca — same window: no residue. The one candidate second-site — `bundle.rs:311`'s direct `builtin_kind::definition()` call, same shape as the bug `feef754` fixed — checked and ruled not the same class: its own module doc states `bundle` "never reads the surface it composes over," always the embedded default by design, no overlay to consult.
- Posture swept through: full src/ list covered; sdk/src/assembly.ts, sdk/src/builtins.ts, and sdk/src/claude-code.ts covered too — mid-rotation, unchanged; sdk/src/contract.ts is the tree-order candidate next.
- This tick: POST-SHIP RECONCILIATION. Audited + swept window bfe147c..a4926ca. Sizing normal (`.flume/metrics.jsonl`: 31-56 turns/entry, no bails, no reverts). Both cursors advanced.
- Queue: 3 pending, 1 open (IMPORT-NESTED-FILE-DOT-ROOT-STRIP-PREFIX), 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — untouched this tick). Refactor: 0 live. Friction: 1 live (plan-null-byte-grep-blindspot.md, human's to read). Amendments: 0 live. Inbox: 0 notes.

Plan continues: after-build — the sole remaining plan job is the posture-sweep rotation (sdk/src/contract.ts next candidate), but IMPORT-NESTED-FILE-DOT-ROOT-STRIP-PREFIX is pickable (open); ready work ships first, the sweep resumes when the wave hands back.
