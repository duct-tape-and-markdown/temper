# Plan state

- **Phase:** reconcile. Verified on disk: **GOV-GRAPH-ACYCLIC shipped** — `graph::acyclic`
  (three-color DFS over resolved reference arcs, one canonicalized cycle finding) is in
  `src/graph.rs` and wired always-on in the `if let Some(layer)` block (`main.rs:198`),
  alongside `graph::check` (route resolution) + `graph::admissibility`. RENAME-ROLLUP-LOCK
  also shipped (roll-up index is `lock.toml`). The field-level numeric `range` predicate is
  on disk too (`contract.rs`/`engine.rs`), so 45-governance's "Also in scope" range is done.
- **Last shipped:** RENAME-ROLLUP-LOCK + GOV-GRAPH-ACYCLIC (`498ab1c`).
- **In flight (1, pickable):** **GOV-GRAPH-DEGREE** — unblocked (its `blockedBy` ACYCLIC
  shipped), now `open`: per-role in/out `degree` bound over the resolved-edge graph.
- **Frontier:** DEGREE completes the graph scope's active predicates (`45-governance.md`);
  remaining 45 items are held back (conditionals) or open loose ends (harness-version
  pinning, `verified_by` wired-and-gating). `decisions-name-alternatives` waits on
  `(decision-marker-predicate)`; spec-kind `references-resolve` is a `temper.toml`
  dogfood-config task, not engine code. Watch `20-surface.md`'s `temper.toml`-root /
  `.temper/`-contents topology for further work.
- **Inbox:** empty (drained prior tick). No open question changed.

Plan continues: no — queue reconciled, DEGREE unblocked and pickable, inbox empty; hand to build.
