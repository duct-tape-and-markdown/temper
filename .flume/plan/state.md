# Plan state

- Spec derived through: 4adb1fb
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: model done — formats next
- This tick: SPEC DELTA. Job 1 (inbox/refactor-captures) quiet. Job 2 was
  live: 4adb1fb (`specs: two postures join the page`) sits past the
  `f7d870c` cursor — it is 4adb1fb's own child, so it was already on disk
  when last tick (a83fe23) ran; that tick took POSTURE SWEEP instead
  (jumping job order) and left the cursor stale at f7d870c. Not
  re-litigated — corrected by servicing the still-live delta now, before
  resuming rotation.
  4adb1fb adds two `engineering.md` sections — "Derived state is computed,
  never stored beside its source" and "The fix lands at the mechanism" —
  from John pointing `/simplify`'s four lenses at the unwinding effort and
  finding two (reuse, efficiency) already encoded and two with no home.
  Process-doc addition, no Decision (`specs/decisions/` newest is still
  0040, confirmed on disk), so no Consequences checklist applies. Routed
  with no new entries, same shape as 6b80e24's routing (7132213):
  - "Derived state is computed" is the code-level twin of pipeline.md's
    already-routed "Emit — derived facts are computed, never authored
    twice"; the commit body names no unresolved incident and the pending
    queue holds no matching live report.
  - "The fix lands at the mechanism" cites its own already-shipped proof
    in its own text — the discovery-override precedent (decision 0034),
    predating this codification.
  Ongoing enforcement for both is the posture sweep, which already reads
  engineering.md live (b8fc7ca) — no prompt edit owed. The model
  subsystem's sweep (a83fe23, this tick's parent commit) ran with both
  sections already in the corpus, so the rotation position (`formats`
  next) is unaffected by this routing. Cursor advances to 4adb1fb, the
  newest `specs/` commit — delta fully closed. Audit/sweep cursors held
  verbatim at 60faee0 (job order precedent 0356dba): `git log
  60faee0..HEAD -- src/ sdk/src/ tests/` is empty, so holding skips
  nothing.
- Queue: 12 pending, all unchanged by this tick (pending.json untouched) —
  3 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-
  COLLAPSE, FRONTMATTER-TEST-SYNTHETIC-KINDS, ROSTER-BUILTIN-KIND-
  NARROWING-RELOCATE; all disjoint files), 7 chained blockedBy
  (DRIFT-LOCK-ROW-WALK-CONSOLIDATION → DRIFT-EMIT-LOCK-PARSE-HOIST →
  PLACEMENT-MODULE-EXTRACTION → EXTRACT-FOUNDATION-BOUNDARY-RESTORE →
  KIND-ZERO-CONSUMER-EXPORTS-PRUNE → CONTRACT-DECLARED-KEYS-EXHAUSTIVE-
  MATCH → CONTRACT-REQUIRE-SECTIONS-ROUNDTRIP), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER). Open forks:
  (multi-harness-projection), (lazy-grounds) unchanged. No live refactor
  captures; inbox empty.

Plan continues: yes — posture sweep resumes at `formats`
(`frontmatter`/`document`/`json_manifest`/`toml_document`), the roster's
next subsystem, once nothing above it is live.
