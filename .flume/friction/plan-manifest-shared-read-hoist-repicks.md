## Symptom

`GATE-MANIFEST-SHARED-READ-HOIST` (shipped 83fbdd5, `chore(flume)` 1ae5def)
needed **6 build-phase attempts** before landing — `.flume/metrics.jsonl`
shows turns 198, 194, 146, 164, 132, 124 across six rows for the one entry,
every other entry in the same window (adf69b3..7173a59, 37 src-touching
ticks) landing in 1 attempt.

The shipped diff threads a new `ManifestCache` through nine call sites
(`gate()`, `explain()`, `resolve_kind_units`, `kind_units_and_features`,
`kind_features`, `assemble_lock_family`, `read_dial`, `manifest_units`,
`coverage_note`) plus a new counter (`manifest_read_count`) and its pinning
tests — a genuinely wide single-entry blast radius by the shape of the
change, not obviously a merge-conflict thrash (no footprint-record commit
exists for this tag, unlike `MAIN-EXPLAIN-READ-CONSOLIDATE`'s recorded
merge failure in the same window).

## Cost this tick

None to plan directly — the tag shipped clean and no live pending entry
depends on it. Read cost only: cross-referencing metrics.jsonl against the
shipped diff to distinguish "oversized entry" from "merge-conflict thrash"
took most of this tick's reconciliation-audit time, since no signal in
pending.json or metrics.jsonl itself labels *why* an entry re-ran.

## Suggested fix

Two candidate angles, drainer's call which (or both):
1. A "threads a new param/cache through N call sites" shape is a derivation
   smell worth naming in `pending-entry.md`'s sizing bullet explicitly (it
   already says "the derivation-time proxy is the `files[]` blast radius" —
   this could sharpen to "N call sites touched, not just N files").
2. `metrics.jsonl` rows carry no outcome/reason field distinguishing a clean
   single-session retry (context exhaustion, a test iteration loop) from a
   merge-reverted re-pick — the footprint-record mechanism
   (`observedFiles`) only fires on some failures (`1c1b025`'s note). If
   build already knows which case it hit, stamping it on the metrics row
   would make this kind of glance mechanical instead of forensic.
