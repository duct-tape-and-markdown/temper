# Plan state

- Spec derived through: 6b80e24
- Audited through: 60faee0
- Residue swept through: 60faee0
- Posture swept through: absent — rotation never initialized, owed once jobs
  1-3 are quiet
- This tick: SPEC DELTA. HEAD moved past the tick's starting snapshot (two
  commits landed mid-session: 6b80e24 `specs:` and b8fc7ca `chore(flume):`,
  neither present when this tick began) — re-oriented off live disk rather
  than the stale snapshot, per job order inbox empty/no refactor
  captures/no src-tests-sdk window past 60faee0 leaves the spec delta the
  first live input. 6b80e24 adds three engineering.md sections — "A green
  verdict is proven non-vacuous", "A fix ships the test that would have
  caught it", "An export earns its consumer" — a process-doc addition, not a
  model Decision, so no Consequences checklist. Routed, no new entries:
  - Vacuity: the rule's own text names three field incidents; two are
    disk-confirmed shipped fixes predating this codification —
    a28e2c6 (EMBEDDED-CLAUSE-BODY-VACUITY-FENCE, fences a body-shaped clause
    bound to an embedded kind at admissibility rather than reading a zeroed
    body as a pass) and 536dd48 (the coverage advisory reclassified by
    present keys, not the registry it used to read instead of the file).
  - Regression pins: stated as the cost section's existing "lands with its
    count-pin" discipline (810da42, already routed) generalized past
    performance — no retroactive incident, binds future entry authoring;
    the pending-entry schema already requires `tests[]` per entry.
  - Export-consumer: the rule's text names the incident "twice" —
    6605bf5 (SDK-ROOT-EXPORT-CLOSURE, closed the SDK root export face) and
    e5de411 (SEAM-EXPORTS-RETRACT, retracted ten unconsumed seam exports),
    both disk-confirmed shipped.
  Ongoing enforcement for all three is the posture sweep, not a one-off
  entry: b8fc7ca (landed the same session, before this tick started)
  rewrote the sweep job to read "every section of engineering.md as it
  reads this tick" rather than a named list, so the three new sections are
  live in the rotation with no prompt edit owed. Cursor advances to 6b80e24
  (the last commit touching specs/; b8fc7ca touches only
  `.flume/prompts/plan.md`, not specs/). Audit/sweep cursors held verbatim
  at 60faee0 — job order precedent (0356dba): only the serviced job's
  cursor moves; window 60faee0..HEAD (b8fc7ca) still touches no
  src/tests/sdk/src (confirmed), so nothing was skipped by holding.
  Side note: read src/drift.rs in full while re-orienting (moot once the
  spec delta pre-empted posture sweep as the tick's job) and found a real
  "one job, one home" triplication — three near-identical lock-row walks
  (`read_prior_provenance` 1918, `config_stale` 2342, `emit_owned_targets`
  2614) — captured to
  `.flume/refactor/plan-drift-lock-row-walk-triplication.md` rather than
  filed, since posture sweep (this tick's non-job) owns that verdict, not
  spec-delta routing.
- Queue: 3 pending — 1 pickable OPEN (DISCOVERY-INFALLIBLE-RESULT-COLLAPSE),
  2 parked on human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER).
  Open forks: (multi-harness-projection), (lazy-grounds).

Plan continues: yes — posture sweep is owed next: jobs 1-3 are quiet through
current HEAD (inbox empty, spec delta now fully routed to 6b80e24, no
src/tests/sdk window since 60faee0) and `Posture swept through:` is still
absent — rotation starts at its first subsystem, `drift` (with the
drift.rs finding above already captured to save that tick a re-read).
