# Plan state

- Spec derived through: 4adb1fb
- Audited through: 1f6afe5
- Residue swept through: 1f6afe5
- Posture swept through: HEAD (this commit) — the re-verification pass
  1f6afe5's ship opened (pipeline, judges, provider all touched) is
  complete: pipeline quiet, judges filed work (posture-sweep judges
  tick), provider re-verified this tick and is quiet too. The new
  rotation cycle (foundation → model → formats → pipeline → judges →
  provider → verbs) opens fresh at foundation next tick.
- This tick: POSTURE SWEEP — job 4. Re-verified the provider subsystem
  (`src/builtin.rs`, `src/builtin_kind.rs`; the SDK's `sdk/src/builtins.ts`/
  `claude-code.ts` are untouched in the window) — `git log 04cbd6d..HEAD --
  src/builtin.rs src/builtin_kind.rs sdk/src/builtins.ts sdk/src/claude-code.ts`
  shows only 84197e5 (BUILTIN-KIND-QUALIFIED-ZERO-CONSUMER-PRUNE's ship,
  already reconciled at 15d4cca). Read both files whole against
  engineering.md's sections. Quiet — no new entry filed:
  - The one live issue already tracked in this pair
    (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE's Result-wrapped
    `definition`/`definitions`) is unchanged; every cited line in that
    entry (kind.rs 16-19/508-524 plus all builtin_kind.rs test call
    sites: 588, 650, 684, 689, 713, 748, 794, 834, 874, 808, 941, 999,
    1022, 1059) re-verified resolving exactly at HEAD — no rescoping
    needed.
  - `manifest_members` (extract.rs:1015-1042) and the mirrored emit-side
    write loop (drift.rs:988-1024) each branch `if collection ==
    CollectionKeyPath::HooksEvent.collection_key()` / `EnabledPlugins`
    then fall through to one generic path for the rest — checked against
    the shared-concept exhaustive-match bar (engineering.md, "A shared
    concept is one type") and found to be the declared exception instead
    ("The fix lands at the mechanism": a documented, cited two-of-four
    special case with an explicit "every other collection" fallthrough,
    extract.rs:1010-1011 — not a bare `_` over the enum). No entry filed.
  - builtin_kind.rs's whole pub surface (`contract`, `contracts`,
    `definition`, `definitions`, `skill_features`, `rule_features`,
    `features`) grep-verified to have a real caller outside its own
    module — no zero-consumer export found.
- Queue: 28 pending, 8 pickable OPEN (TAP-LOG-FILENAME-ZERO-CONSUMER-PRUNE,
  ADDRESS-FIELDPATH-SPELLING-ZERO-CONSUMER-PRUNE,
  KIND-MEMBER-DOCUMENT-ZERO-CONSUMER-PRUNE,
  FRONTMATTER-COMPANIONS-ZERO-CONSUMER-PRUNE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH, DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION), 16 chained blockedBy, 4 parked on
  human action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE, NORMALIZE-PATH-SUBSYSTEM-
  PLACEMENT). Unchanged this tick — the sweep found nothing to file.
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — the new posture-sweep rotation cycle opens fresh
at foundation next tick.
