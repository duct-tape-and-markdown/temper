# Plan state

- Spec derived through: 20a6f54 — unchanged, no spec/ commits past it.
- Audited through: 729b0cad — unchanged, no src/tests/sdk commits past it.
- Residue swept through: 729b0cad — unchanged, same.
- Posture swept through: 97d0241 — mid-rotation: src/compose.rs covered
  this tick, quiet-on-clean. Frontier remaining: src/main.rs, src/read.rs,
  src/install.rs, sdk/src/builtins.ts.
- This tick: POSTURE SWEEP — neighborhood src/compose.rs (+ immediate
  imports builtin_kind, contract, dial, document, drift, extract,
  frontmatter, graph, import, json_manifest, kind, layout, toml_document)
  read against engineering.md + architecture.md. No embedded-provider-
  knowledge literal: the `hooks.<Event>` key-path stays doc-comment prose
  here, the live collection key is sourced via `address.key_path` off
  `kind::CollectionAddress`, never a bare literal. Every pub export has an
  out-of-module consumer (rg-verified, 20/20 functions). `Verifier`'s two
  variants (Script/Telemetry) are both constructed (roster.rs, drift.rs)
  and matched exhaustively. `partition_kind_rows`'s discarded `_collisions`
  in `declared_kinds_with_overlaid` mirrors the same discard already at
  main.rs/read.rs call sites (gate.rs is the one consumer that uses it) —
  not new residue. Quiet-on-clean, no findings.
- Queue: 3 pending — 0 open, 1 parked, 2 deferred. Open forks: 2, unchanged.
  Friction: 2, unchanged. Amendments: 0. Inbox: 0.

Plan continues: yes — posture rotation is open with 4 frontier modules left
(src/main.rs, src/read.rs, src/install.rs, sdk/src/builtins.ts) and the
queue has no pickable entries, so plan takes the next tick.
