# Plan state

- Spec derived through: 53df138 — unchanged; its only specs/ diff (the
  normalize_path Invariants amendment) is fully routed, nothing past it
  remains un-derived.
- Audited through: d40a9f8
- Residue swept through: d40a9f8
- Posture swept through: 5af93d9 — foundation ticked, fresh cycle
  opens (foundation → model → formats → pipeline → judges → provider
  → verbs).
- This tick: POSTURE SWEEP — job 4, foundation subsystem (the roster's
  first, per last tick's re-arm note). Read check.rs, extract.rs,
  hash.rs, address.rs, tap.rs, json_splice.rs whole against
  engineering.md's lenses (one job one home, a shared concept is one
  type, an export earns its consumer, dead plumbing). Grep-verified
  every pub/pub(crate) item in hash.rs, json_splice.rs, tap.rs,
  address.rs to a live consumer (a test counts: TapEvent/TapRecord/
  LogReadout/TAP_RECORD_VERSION all read by tests/tap.rs and
  tests/read_verbs.rs; sha256_hex, apply_edits/object_shape/
  array_shape/insert_member/append_element/pretty_at, FieldPath's
  parse/is_bare_name/head_name/split_leaf/locate all called from
  outside their module). check.rs's one zero-consumer const
  (ANNOUNCEMENT_HEADING) is already the open
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE entry; extract.rs's
  upward crate::drift::NestedMemberRow/crate::kind imports are already
  the open (blockedBy-chained) EXTRACT-FOUNDATION-BOUNDARY-RESTORE
  entry (nested_members_from_rows/embedded_member_from_row →
  drift.rs; manifest_members/entry_fields/enablement_*/hook_* →
  json_manifest.rs) — no further upward dependency, non-exhaustive
  match over a shared enum, or duplicate-job residue found beyond it.
  One stale module doc found and left riding, never filed as its own
  entry (the ride-only rule): json_splice.rs's header claims
  install.rs as "the sole consumer," but json_manifest.rs now calls
  apply_edits/object_shape/insert_member/pretty_at too (json_manifest.rs
  is not currently in any open or blockedBy entry's files, so nothing
  to ride this tick). Quiet-on-clean: no new entry filed, rotation
  advances alone.
- Queue: 27 pending, unchanged — 6 pickable OPEN
  (BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  JSON-MANIFEST-READ-DECODE-CONSOLIDATE,
  TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE,
  COMPOSE-DIAL-SEVERITY-LABEL-CONSOLIDATE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE,
  DRIFT-LOCK-ROW-WALK-CONSOLIDATION — pairwise file-disjoint,
  re-verified this tick), 19 chained blockedBy (unchanged links, all
  still resolve to live tags), 2 parked on human action
  (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER — reasons
  unchanged, untouched this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — posture sweep resumes at `model`
(`kind`/`contract`/`compose`/`schema`/`roster`), the roster's next
subsystem. `git log 1c5b0a9..HEAD -- src/kind.rs src/contract.rs
src/compose.rs src/schema.rs src/roster.rs` is empty (model's own
last sweep, 1c5b0a9, already covers this window), so next tick skips
forward without a re-read unless a later commit lands first.
