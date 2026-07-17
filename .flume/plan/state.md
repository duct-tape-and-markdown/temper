# Plan state

- Spec derived through: 63e1f22
- Audited through: b85df4a
- Residue swept through: b85df4a
- This tick: DERIVE spec delta 63e1f22 (decision 0036, "the machine's own file
  gets a kind") — one contained slice, one atomic entry **SETTINGS-LOCAL-KIND**.
  0036's Consequences carries ONE derivation bullet, routed:
  - "Plan derives the entry (kind, default contract with the documented-profile
    clauses the settings docs settle, fixture under the discovery override)" →
    **SETTINGS-LOCAL-KIND**. The `settings-local` shipped kind:
    `.claude/settings.local.json` as a `json-document` at the **local**
    commitment class — the plugin-manifest json-document posture crossed with
    dial's local locus. Two-sided by construction (`sdk.md` seam): SDK
    authoring def (`sdk/src/builtins.ts` + provider export) + Rust read-side def
    (`src/builtin_kind.rs` `all_kinds()`) + `builtin_lock.toml` re-derive +
    `builtin_lock.rs`'s two hard-coded kind lists. `src/{builtin,contract,kind}.rs`
    unchanged — the machinery (`Commitment::Local`, `Format::JsonDocument`,
    discovery override) already ships; only data is added.
  - "builtins.md grows to eleven / sits outside the domain partition" → already
    in the spec body (63e1f22), not code.
  - "The (settings-local-kind) record deletes" → verified ALREADY absent from
    open-questions.md (routed on the 0032/0036 line); no deletion owed this tick.
  Disjointness: SETTINGS-LOCAL-KIND overlaps EXTENT-PREDICATE on
  `builtin_lock.toml`, `builtins.ts`, `builtins.test.ts`, `lock_declaration_rows.rs`
  (both regenerate the lock) → serialized `blockedBy: EXTENT-PREDICATE`
  (`pending-entry` rule). No overlap with the two parked entries.
  0037 (6d2cca6, typed verifier — a multi-part capability: tap verb, telemetry
  declaration + hook projection, local-locus log kind, field strand, verifier-type
  resolution) is NOT derived this tick — its own slice, next.
- Queue: 4 entries — EXTENT-PREDICATE **pickable** (gate:open); SETTINGS-LOCAL-KIND
  (blockedBy EXTENT) + 2 parked (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER).
  DAG-disjoint. No fork rest.

Plan continues: yes — spec delta still live (6d2cca6 / 0037 un-derived, one
slice per tick), and post-ship reconcile of b85df4a..HEAD (3 build commits)
still pending below it.
