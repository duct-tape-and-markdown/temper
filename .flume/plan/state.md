# Plan state

- Spec derived through: f87cc0c
- Audited through: e50d082
- Residue swept through: e50d082
- This tick: Ship audit (job 3) was quiet first — the one commit past the old
  audited-through cursor (01b5205) was e50d082, plan's own prior tick,
  touching only `.flume/plan/*`, no src/tests/sdk. Cursor advanced to HEAD.
  Residue sweep (job 4) then ran fresh against src/, tests/, sdk/ beyond the
  5 already-queued entries (dispatched an Explore sweep in parallel with a
  manual grep pass). Two new classes found and verified on disk:
  (1) 10 in-src `#[cfg(test)]` modules each define a byte-identical private
  `tmpdir(label)` helper; the shipped TMPDIR-HELPER-CONSOLIDATE(mainrs)/
  (kindrs) entries only renamed 2 of them to match a "shared convention"
  without ever merging into one home. Filed IN-SRC-TMPDIR-HELPER-CONSOLIDATE
  for the 8 sites disjoint from open work (check.rs, bundle.rs, main.rs,
  import.rs, builtin_kind.rs, install.rs, kind.rs, frontmatter.rs, plus new
  src/test_support.rs + src/lib.rs); drift.rs and coverage_note.rs also
  carry the copy but are deferred, entangled with open
  PATH-SEP-NORMALIZE-CONSOLIDATE's file scope. (2) src/extract.rs's
  body_headings/body_sections/body_at_imports each run an identical
  fence-open/close-tracking match (each site's own doc comment admits it
  mirrors body_headings) instead of one shared helper. Filed
  EXTRACT-FENCE-SCAN-CONSOLIDATE, fully disjoint (no other entry touches
  extract.rs). Explore sweep found nothing else beyond the already-queued
  classes (checked sha256_hex, compile_glob, path/line-ending normalizers,
  document.rs's `+++`-closer scan vs frontmatter::closing_delimiter — ruled
  out, differing error/round-trip contract — and the SDK, clean). Residue
  swept through cursor advances to HEAD (e50d082).
- Queue: PATH-SEP-NORMALIZE-CONSOLIDATE, SDK-SORT-COMPARATOR-CONSOLIDATE,
  IN-SRC-TMPDIR-HELPER-CONSOLIDATE, EXTRACT-FENCE-SCAN-CONSOLIDATE,
  TEST-HELPER-DUPES-CONSOLIDATE(recordwriters) all open and pairwise
  disjoint, pickable in parallel — TEST-HELPER-DUPES-CONSOLIDATE
  (lockunitbuilders) (blockedBy recordwriters) — PACKAGING-CHANNELS (parked,
  condition unchanged).

Plan continues: yes — every input (inbox, spec delta, ship audit, residue
sweep) is now current; next tick is the quiet closing pass (job 5).
