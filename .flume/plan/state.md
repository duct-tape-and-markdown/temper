# Plan state

- Spec derived through: c9d11d5 — routed in full, 0 new entries; see commit body.
- Audited through: 126c264 — unchanged; `git log 126c264..HEAD -- src/ tests/ sdk/` is empty.
- Residue swept through: 126c264 — unchanged, same reason.
- Posture swept through: mid-rotation, at src/compose.rs — eighth module of the
  c9d11d5 re-arm rotation (alphabetical order; neighborhood is compose.rs plus
  its crate-internal imports: builtin_kind, contract, dial, document, drift,
  extract, frontmatter, graph, import, json_manifest, kind, layout,
  toml_document). Verdict: one violation, filed — `edge_fields_by_kind` (1076)
  and `embedded_member_features` (1103) are `pub fn` with zero callers outside
  compose.rs itself (rg-verified against src/, tests/, main.rs), unlike every
  other module-internal helper in the file (row_relocates_builtin,
  layer_lock_path, read_dial, …), which is already plain `fn` — "An export
  earns its consumer" residue, filed as COMPOSE-ZERO-CONSUMER-VISIBILITY-PRUNE.
  The already-tracked stale `manifest_units` cite (556, open-questions.md)
  remains untouched by this entry, so it still rides forward. `src/contract.rs`
  next in the frontier.
- This tick: POSTURE SWEEP src/compose.rs — 1 pending entry filed
  (COMPOSE-ZERO-CONSUMER-VISIBILITY-PRUNE).
- Queue: 3 pending — 1 open (COMPOSE-ZERO-CONSUMER-VISIBILITY-PRUNE), 1 deferred
  (GUIDANCE-FIELD-DECLARATION-CHANNEL), 1 parked (PACKAGING-CHANNELS-REMAINDER);
  0 open questions unresolved by this queue. Open forks: 2, unchanged.
  Friction: 0. Amendments: 0. Inbox: 0.

Plan continues: after-build — the only remaining live job is the posture
rotation (frontier non-empty: contract.rs onward across src/, sdk/src/, tests/
remain unswept this window), and a pickable entry now exists, so the wave
ships it first and the sweep resumes when it hands back.
