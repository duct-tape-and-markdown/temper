# Plan state

- Spec derived through: 506c34c — unchanged, not this tick's job.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: 2d1c5a6 — verbs ticked this tick and closes
  the rotation pass (foundation done, model done, formats done,
  pipeline quiet-on-clean, judges bulk-skipped, provider done last
  tick, verbs this tick — 4 new findings). All seven subsystems from
  architecture.md's codemap now covered. A fresh cycle (foundation →
  model → formats → pipeline → judges → provider → verbs) opens next
  time the sweep re-arms.
- This tick: POSTURE SWEEP — verbs subsystem (`src/main.rs`,
  `src/install.rs`, `src/bundle.rs`, `src/lib.rs`, `src/test_support.rs`),
  touched since its own last sweep (8003aad) by 278ae4c
  (install-projection-match-consolidate) and 404b73a
  (extract-foundation-boundary-restore). Read both pages
  (specs/process/engineering.md whole, specs/process/architecture.md's
  codemap) and the full current state of all five files (a
  general-purpose agent did the first pass census — cross-checked
  against every already-queued entry touching these files and the
  open-questions "Kept on purpose" list — independently re-verified on
  disk before filing) against every engineering.md section plus the
  cohesion/dead-plumbing/embedded-provider-knowledge sweep lenses.
  lib.rs/test_support.rs clean. Four new findings, all mechanical:
  - **INSTALL-ERROR-ZERO-CONSUMER-PRUNE** — install.rs's `InstallError`
    enum (168) is `pub` with zero consumer outside its own module; every
    fallible pub fn in the file returns `miette::Result<T>`, never
    naming the concrete type. Same shape as the shipped GRAPH-WORLD-
    ZERO-CONSUMER-PRUNE precedent. Serialized behind GUARD-DECLARED-
    LOCUS-FILTER, the tail of the existing install.rs chain.
  - **BUNDLE-ERROR-ZERO-CONSUMER-PRUNE** — bundle.rs's `BundleError`
    enum (140) is the identical zero-consumer shape. Filed open.
  - **BUNDLE-MANIFEST-PATH-GOVERNS-DERIVE** — bundle.rs's `write_member`
    (309) takes a hardcoded `.claude-plugin/plugin.json` /
    `marketplace.json` literal at each call site instead of deriving it
    from the kind's own `Governs` locus, already declared and cited in
    builtin_kind.rs (376-379, 406-408) — two sources of truth for one
    manifest's location. Serialized behind BUNDLE-ERROR-ZERO-CONSUMER-
    PRUNE for shared bundle.rs safety.
  - **MAIN-GUARD-DECLARATIONS-DOUBLE-READ-HOIST** — `Command::Guard`'s
    handler independently calls `drift::read_declarations` twice
    (via `mode_from_lock` at 382 and `guarded_manifests` at 385) —
    a full re-read-and-reparse of lock.toml within one CLI invocation,
    untouched by any of the five queued `*-PARSE-HOIST` entries (all
    scoped to `gate()`/`explain()`, verified against their files[]).
    `guarded_manifests` does nothing else with `workspace_dir`, so it
    can take `&Declarations` directly — the sibling
    `mode_from_declarations` already exists for exactly this shape.
    No runtime count-pin available (`read_declarations` doesn't feed
    drift's `lock_read_count`/`lock_parse_count` counters, the same gap
    DRIFT-SOURCE-DEP-PARSE-HOIST's own pin will face); proven
    structurally instead, the named exception per "A fix ships the test
    that would have caught it." Serialized behind DRIFT-SOURCE-DEP-
    PARSE-HOIST since it and MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT are
    already both open over main.rs — chaining here avoids a third
    simultaneous open edit to the same file.
  - Checked clean or already covered, not re-filed: full export census
    across all five files (every remaining pub item has a real outside
    caller); no `_ =>` wildcards over a shared enum in these files; the
    "main carries dispatch only" invariant already has its own tracked
    exceptions (MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT,
    MAIN-CORPUS-ASSEMBLY-TO-COMPOSE, MAIN-JUDGE-VERB-HOME-RULING);
    install.rs's GUARD_PATH_MATCH regex (714) is a libraries-before-
    hand-rolls candidate but collides with GUARD-DECLARED-LOCUS-FILTER's
    own ruling to keep it scoped as-is — not re-opened; the orphaned
    `placement_lines` doc comment (install.rs 1646-1652, open-questions'
    "one stale cite, ride-only" record) confirmed still present,
    unchanged, riding per the documented rule, not re-reported; no fresh
    embedded-provider-knowledge leak beyond the governs-path finding
    above.
- Queue: 49 pending (+4 this tick: INSTALL-ERROR-ZERO-CONSUMER-PRUNE,
  BUNDLE-ERROR-ZERO-CONSUMER-PRUNE, BUNDLE-MANIFEST-PATH-GOVERNS-DERIVE,
  MAIN-GUARD-DECLARATIONS-DOUBLE-READ-HOIST). 10 pickable OPEN, 36
  chained blockedBy, 3 parked on human action. This tick's new open
  entry (BUNDLE-ERROR-ZERO-CONSUMER-PRUNE) is file-disjoint from every
  other open entry — verified. **Pre-existing gap surfaced, not this
  tick's to fix**: DRIFT-SOURCE-DEP-PARSE-HOIST and MAIN-LOCK-ROW-
  CONSTRUCTORS-TO-DRIFT are both already `open` and both edit
  src/main.rs and src/drift.rs — pending-entry.md's "Disjoint, or
  serialized" bar unmet since before this tick (neither entry's own
  filing tick is this one). MAIN-GUARD-DECLARATIONS-DOUBLE-READ-HOIST
  was deliberately chained behind DRIFT-SOURCE-DEP-PARSE-HOIST rather
  than left open, to avoid adding a third simultaneous main.rs edit —
  but the pre-existing pair stands unresolved; a future tick should
  serialize one behind the other. Open forks unchanged:
  (multi-harness-projection), (lazy-grounds), neither touched this
  tick. Refactor captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: no — the posture rotation closes this tick (all seven
subsystems covered this cycle), no spec delta past 506c34c, and no
post-ship reconciliation window past 64828d9. Next wake re-arms the
rotation once a forward window touches a subsystem, or a spec/post-ship
input goes live.
