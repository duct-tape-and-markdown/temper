# Plan state

- Spec derived through: 5945405
- Audited through: 2b82ebd
- Residue swept through: 2b82ebd
- This tick: Spec delta f4189c3, third slice — 0008 settings-write-format-preserving
  → SETTINGS-FORMAT-PRESERVING (blockedBy GUARD-OWNPATH: shares src/install.rs +
  tests/install.rs). Install's hook merge into an existing settings.json must be
  format-preserving (existing keys/order/formatting survive; one-hunk diff) instead
  of re-serializing the whole file. 0009 (module-relative path resolution, SDK
  emit.ts + scaffold) / 0011 (documented-capability vocabulary — user-invoked +
  hook/settings kinds + unclaimed-entry advisory) still unrouted — f4189c3 NOT
  fully routed, spec cursor stays at 5945405. Audited/Residue cursors copied
  forward verbatim (not serviced this tick).
- Queue: 5 — EMIT-REAP-ORPHANS open (drift.rs), EMIT-LF-NORMALIZE blockedBy
  EMIT-REAP-ORPHANS (same drift.rs+emit.rs), GUARD-OWNPATH blockedBy
  PATH-SEP-NORMALIZE (blocker SHIPPED 2efd00b — ship audit unblocks it),
  SETTINGS-FORMAT-PRESERVING blockedBy GUARD-OWNPATH (same install.rs +
  tests/install.rs), PACKAGING-CHANNELS parked (human release creds). Chains:
  drift.rs / install.rs+kind.rs / package.json.

Plan continues: yes — spec delta still live (f4189c3 rulings 0009/0011 unrouted);
ship audit also trailing (PATH-SEP-NORMALIZE shipped past Audited-through 2b82ebd).
