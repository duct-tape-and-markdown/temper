# Plan state

- Spec derived through: 5945405
- Audited through: 2b82ebd
- Residue swept through: 2b82ebd
- This tick: Spec delta f4189c3, fourth slice — 0009 path-references-module-relative
  → MODULE-RELATIVE-PATHS (blockedBy SETTINGS-FORMAT-PRESERVING: shares src/install.rs
  + tests/install.rs). A file() asset resolves against one cwd/baseDir
  (emit.ts fileSourcePath:147), not the module that stated it; scaffold's `../`
  paths (install.rs relative_to_workspace:957) work only because member modules
  happen to sit at .temper/<kind>/ — recut resolution to the stating module
  (ESM import.meta.url), move scaffold output + SDK docs with it. 0011
  (documented-capability vocabulary — user-invoked value + hook/settings kinds +
  unclaimed-entry advisory; a family, hook/settings kinds intersect the open
  (json-projection-format) fork) still unrouted — f4189c3 NOT fully routed, spec
  cursor stays 5945405. Audited/Residue cursors copied forward verbatim (not
  serviced this tick).
- Queue: 6 — EMIT-REAP-ORPHANS open (drift.rs), EMIT-LF-NORMALIZE blockedBy
  EMIT-REAP-ORPHANS (same drift.rs+emit.rs), GUARD-OWNPATH blockedBy
  PATH-SEP-NORMALIZE (blocker SHIPPED 2efd00b — ship audit unblocks it),
  SETTINGS-FORMAT-PRESERVING blockedBy GUARD-OWNPATH, MODULE-RELATIVE-PATHS
  blockedBy SETTINGS-FORMAT-PRESERVING (install.rs chain tail; also SDK
  prose.ts+emit.ts+emit.test.ts, owned by no other entry), PACKAGING-CHANNELS
  parked (human release creds). Chains: drift.rs / install.rs+kind.rs (+SDK) /
  package.json.

Plan continues: yes — spec delta still live (f4189c3 ruling 0011 unrouted); ship
audit also trailing (PATH-SEP-NORMALIZE shipped past Audited-through 2b82ebd).
