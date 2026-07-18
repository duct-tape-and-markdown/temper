# Plan state

- Spec derived through: 64828d9 — unchanged, not this tick's job. Live
  input for the next tick: 506c34c (specs: 0042 — "the entry declares
  its shape") lands past this cursor and is not yet routed.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: judges next (mid-rotation) — pipeline read
  and swept last tick (ab01fb4); not this tick's job.
- This tick: INBOX. Two commits landed on the repo mid-session, after
  this tick's orientation snapshot was taken and after last tick's
  posture-sweep commit: 506c34c (a human spec commit, decision 0042)
  and ff57f21 (`chore(flume+harness)`, human territory — applied the
  0042 frame one level up, at subsystem level, ruling three
  embedded-provider-knowledge instances via the inbox, and added the
  lens itself to `.claude/rules/posture-sweep.md`/`.temper/rules/
  posture-sweep.md` for future sweeps). Re-oriented off live disk
  state rather than the stale snapshot: `.flume/inbox.md` held three
  RULED notes (observed ab01fb4), outranking the newly-live spec delta
  in job order, so inbox is this tick's job, spec delta next tick's.
  Routed all three, each pre-decided (standing delegation) so each
  filed `gate: open` (or serialized behind an existing open entry
  sharing its file) rather than re-litigated:
  - TAP-PAYLOAD-SCHEMA-SPLIT — tap.rs's `record_from_payload`
    (126-171) inlines Claude Code's hook-payload schema (event names,
    `tool_input.skill` field path) in a foundation module; the
    classification moves to the provider face, record vocabulary/IO
    stay. No file conflict with the existing queue (tap.rs untouched
    elsewhere) — filed open.
  - COVERAGE-KNOWN-SURFACES-RELOCATE — coverage_note.rs's
    `KNOWN_SURFACES` registry and its hardcoded `.claude` scan root
    (256) duplicate the provider face's own already-declared `.claude`
    root (`builtin_kind.rs`'s five `Governs` literals); both relocate.
    No open-entry conflict on coverage_note.rs — filed open.
  - GUARD-DECLARED-LOCUS-FILTER — install.rs's `GUARD_PATH_MATCH`
    regex hard-admits only `.claude/`-rooted paths ahead of consulting
    `targets` (the lock's declared emit-owned set), so a declared-locus
    kind governing outside `.claude` (0038 layout kinds) escapes the
    guard silently — a real gap, not just a shape defect. install.rs
    already carries an open entry (INSTALL-GUARD-MANIFEST-MESSAGE-
    PRUNE) chaining through INSTALL-PLACEMENT-KIND-ENUM to
    BUNDLE-INSTALL-SESSION-START-SHAPE-CONSOLIDATE — serialized this
    behind that chain's last install.rs-touching entry
    (pending-entry.md, "Disjoint, or serialized") rather than filed
    open.
  All three per-cite architecture.md's Invariants/Codemap sections —
  the same "did we widen at the wrong level" frame decision 0042
  itself argued, applied one level up to knowledge (a cited external
  fact spelled as a literal) rather than behavior (an identity branch).
  Inbox drained to its template comment; nothing else live there.
- Queue: 41 pending (+3 this tick: TAP-PAYLOAD-SCHEMA-SPLIT open,
  COVERAGE-KNOWN-SURFACES-RELOCATE open, GUARD-DECLARED-LOCUS-FILTER
  blockedBy). 9 pickable OPEN (the prior 7 plus the two new open
  entries), 29 chained blockedBy, 3 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  MAIN-JUDGE-VERB-HOME-RULING). Open forks unchanged: (multi-harness-
  projection), (lazy-grounds), neither touched. Refactor captures: 0
  live. Friction: 0 live. Inbox: 0 notes (drained this tick).

Plan continues: yes — the spec delta (506c34c, decision 0042) is now
the first live input in job order and has not yet been routed; next
tick derives it from its own Consequences checklist.
