# Plan state

- Spec derived through: 506c34c — decision 0042 ("the entry declares
  its shape") routed 506c34c's tick; unchanged, not this tick's job.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: verbs next (mid-rotation) — provider read
  and swept this tick.
- This tick: POSTURE SWEEP. Inbox empty, no refactor captures, no
  spec-delta commits past 506c34c, and `git log 64828d9..HEAD -- src/
  tests/ sdk/` empty (post-ship reconciliation not live) — job 4 was
  the first live input. Rotation: `judges` skip-forwarded last tick
  (quiet since fe3ff3f), landing on `provider`
  (src/builtin.rs, src/builtin_kind.rs, sdk/src/builtins.ts,
  sdk/src/claude-code.ts) — touched since its own last sweep (fe3ff3f)
  by 404b73a (extract-foundation-boundary-restore, narrowed
  builtin_kind.rs's call sites to drift::) and 516f8f6 (collapsed
  definitions()'s Result wrapper). Read both pages
  (specs/process/engineering.md whole, specs/process/architecture.md's
  codemap) and the full current state of all four files (a
  general-purpose agent did the first pass census, independently
  re-verified before filing) against every engineering.md section plus
  the cohesion/dead-plumbing sweep lenses (embedded-provider-knowledge
  is N/A here — this subsystem *is* the provider face the lens
  protects other subsystems from leaking into).
  - One real, verified finding: **BUILTIN-KIND-SURFACE-UNIT-CONSOLIDATE**
    — builtin_kind.rs's inline `#[cfg(test)]` module declares a private
    `surface_unit` (885-901) that is a byte-for-byte body duplicate of
    `tests/common::surface_unit` (mod.rs:460-476), when this same test
    module already imports `crate::test_support::tmpdir` (574) — the
    crate's own designated shared-fixture home for in-src unit tests
    (test_support.rs's own header names exactly this job). Filed
    against "One job, one home" (test-scaffolding bullet), blockedBy
    KIND-ENTRY-SHAPE-DATA-DECLARE for shared builtin_kind.rs safety
    (pending-entry.md, "Disjoint, or serialized") — TAP-PAYLOAD-SCHEMA-
    SPLIT also touches the file and is open/queue-front, ships first
    regardless, noted rather than double-chained.
  - Checked clean or already covered, not re-filed: export census
    (every pub item in builtin.rs/builtin_kind.rs and every
    builtins.ts/claude-code.ts export has a real outside caller); no
    `match` statements in either Rust file, so no `_ =>` exhaustiveness
    risk; the architecture invariant ("the provider face is data the
    engine loads, never a dependency of the model") holds — kind.rs/
    json_manifest.rs reference builtin_kind::features only via
    doc-comment intra-links, never a real `use`; builtin_kind::
    definitions()'s redundant in-run calls already covered by
    GATE-KIND-UNITS-DOUBLE-RESOLVE-HOIST; builtin.rs's contract_for_kind
    duplicate of compose::default_contract_from_rows already covered by
    BUILTIN-CONTRACT-FOR-KIND-CONSOLIDATE; KIND-ENTRY-SHAPE-DATA-DECLARE/
    JSON-MANIFEST-ENTRY-SHAPE-DISPATCH/GRAPH-ENABLEMENT-FIELD-RETIRE/
    BUILTINS-DEFAULT-CONTRACT-HOLDS-CLOSE all still correctly pending,
    untouched by this tick's window; no fresh stale-cite instances (all
    citation dates 2026-07-15/16/17, consistent with today); no
    collision with any "Kept on purpose" asymmetry.
  - Rotation continued forward in the same tick per the skip-forward
    rule: checked `verbs` (main.rs, install.rs, bundle.rs, lib.rs,
    test_support.rs) against its own last-sweep baseline (8003aad) —
    `git log 8003aad..HEAD` names 278ae4c and 404b73a touching
    install.rs/main.rs, so `verbs` is itself touched, not quiet. The
    rotation does not close this tick; it stops at `provider` (already
    read) and the cursor lands on `verbs` as the next live subsystem —
    unread this tick, per the one-touched-subsystem-per-tick bound.
- Queue: 45 pending (+1 this tick, BUILTIN-KIND-SURFACE-UNIT-CONSOLIDATE,
  blockedBy KIND-ENTRY-SHAPE-DATA-DECLARE). 9 pickable OPEN (unchanged
  — the new entry is blockedBy), 33 chained blockedBy, 3 parked on
  human action. Open forks unchanged: (multi-harness-projection),
  (lazy-grounds), neither touched this tick. Refactor captures: 0 live.
  Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture-sweep rotation is mid-cycle: `verbs`
is touched since its own last sweep (8003aad, by 278ae4c/404b73a) and
is the next live subsystem to read and sweep.
