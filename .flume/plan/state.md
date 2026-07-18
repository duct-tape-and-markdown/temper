# Plan state

- Spec derived through: 64828d9 — unchanged, not this tick's job.
- Audited through: 64828d9 — unchanged, not this tick's job.
- Residue swept through: 64828d9 — unchanged, not this tick's job.
- Posture swept through: pipeline next (mid-rotation) — formats read and
  swept this tick.
- This tick: POSTURE SWEEP. Inbox and spec delta both drained (empty,
  re-verified this tick: `git log 64828d9..HEAD -- specs/` and the
  inbox file both empty); audit/residue cursors current (`git log
  64828d9..HEAD -- src/ sdk/src/ tests/` empty — every commit since is
  a `plan:` commit, none touching code). Rotation continues the cycle
  4d1c261 opened (roster from architecture.md's codemap: foundation,
  model, formats, pipeline, judges, provider, verbs; foundation and
  model swept the prior two ticks). `formats` was already
  known-touched by 404b73a (json_manifest.rs, the manifest-grammar
  functions moved in from extract.rs), so it is this tick's one
  read-and-swept subsystem: all four modules (frontmatter, document,
  json_manifest, toml_document) read in full against every
  engineering.md section and architecture.md's format invariants
  ("formats never know the verbs or judges"; format mechanics are
  engine code, kind is data). Found one new finding: `manifest_members`
  (json_manifest.rs:538, `pub(crate)`) has zero callers anywhere outside
  its own file's `Manifest::parse` and its own inline tests — grep-
  verified, the same zero-consumer shape 404b73a correctly narrowed its
  sibling grammar fns (`enablement_member_fields`/`hook_member_fields`)
  to but left this one `pub(crate)` without a matching outside caller
  ever existing at the new address; kind.rs's `ENABLEMENT_FIELD` doc
  comment intra-doc-links it without calling it. Filed
  JSON-MANIFEST-MEMBERS-ZERO-CONSUMER-PRUNE, serialized behind
  GATE-MANIFEST-SHARED-READ-HOIST (the last existing chain entry
  touching json_manifest.rs). Beyond that: no cohesion split, no dead
  plumbing, no `_`-arm over a shared concept (every `UnitShape` match in
  json_manifest.rs/toml_document.rs is already exhaustive), no
  stored-derived-state. Two already-known stale cites re-confirmed
  unclaimed by any open entry (document.rs's `item_to_json` doc citing
  the deleted `json_to_toml_value`; json_manifest.rs:335's `Manifest::read`
  doc self-referentially citing `extract::manifest_members` after the
  404b73a move) — both still ride-only per open-questions.md, neither
  gets a standalone entry. `pipeline` (drift.rs), `provider`
  (builtin_kind.rs), and `verbs` (main.rs, install.rs) remain touched by
  404b73a too; `judges` stays the one fully-untouched subsystem in this
  window. `pipeline` is next in roster order.
- Queue: 38 pending (+1 this tick: JSON-MANIFEST-MEMBERS-ZERO-CONSUMER-
  PRUNE, blockedBy). 7 pickable OPEN (DRIFT-SOURCE-DEP-PARSE-HOIST,
  INSTALL-GUARD-MANIFEST-MESSAGE-PRUNE, KIND-ZERO-CONSUMER-EXPORTS-PRUNE,
  IMPORT-ROLLUP-WRITER-PLACEMENT, READ-CONTEXT-MEMBER-CITER-GRAIN,
  READ-VERB-STRAND-COHESION, MAIN-LOCK-ROW-CONSTRUCTORS-TO-DRIFT), 28
  chained blockedBy, 3 parked on human action (IMPORT-HOP-CAP-CITE,
  PACKAGING-CHANNELS-REMAINDER, MAIN-JUDGE-VERB-HOME-RULING). Open forks
  unchanged: (multi-harness-projection), (lazy-grounds), neither
  touched. Refactor captures: 0 live. Friction: 0 live. Inbox: 0 notes.

Plan continues: yes — the posture-sweep rotation is mid-cycle
(`pipeline` next) and `pipeline`'s own file (drift.rs) is already
known-touched by 404b73a, so next tick's job is live without a fresh
forward-window check.
