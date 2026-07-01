# Plan state

- **Phase:** reconcile. Verified on disk: **KIND-EDGE-RELATIONSHIPS shipped** — the
  standalone `[[edge]]` construct is fully retired; edges parse from
  `[[kind.<name>.relationships]]` (`compose.rs`, `Edge {field,from,to}`), and
  `graph::check` (route resolution) + `graph::admissibility` are wired in `main.rs`.
  Custom kinds check end-to-end (5 extraction primitives; 16 artifact predicates;
  set-scope `count`/`unique`/`membership`+typed-ref in `roster.rs`).
- **Last shipped:** KIND-EDGE-RELATIONSHIPS (`a2d4cb1`). Tree clean; queue was empty.
- **Filed this tick (3, all parallel-safe):** **RENAME-ROLLUP-LOCK** (`open`) —
  drains the inbox: rename the generated roll-up `author.toml` → `lock.toml`, resolving
  `(rollup-index-rename)` (spec `20-surface.md` was revised to name `lock.toml`; only
  `src/import.rs` writes it). **GOV-GRAPH-ACYCLIC** (`open`) — always-on cycle check
  over the harness edge graph. **GOV-GRAPH-DEGREE** (`blockedBy` ACYCLIC) — per-role
  in/out degree bound. RENAME is disjoint (import.rs); the two graph entries share
  graph.rs/main.rs so DEGREE is serialized behind ACYCLIC.
- **Frontier:** ACYCLIC+DEGREE complete the graph scope (`45-governance.md`). Then
  `decisions-name-alternatives` waits on `(decision-marker-predicate)`; the spec kind's
  `references-resolve` is a `temper.toml` dogfood-config task (the graph tier already
  resolves custom-kind relationship edges), not new engine code. Note: `20-surface.md`'s
  new `temper.toml`-root / `.temper/`-contents topology may imply further work — watch it.
- **Inbox:** drained (rollup-rename routed to RENAME-ROLLUP-LOCK + open-question RESOLVED).

Plan continues: no — three pickable/queued entries filed, inbox drained, open question
resolved; hand to build.
