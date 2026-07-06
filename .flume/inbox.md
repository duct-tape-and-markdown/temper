<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

DERIVED-LOCK CHAIN (decomposition ceremony ran in-session, John + session
07-06; per 50-distribution "Decision: the built-in lock is derived from the
SDK module, never transcribed" + 20-surface's row-identity paragraph). File
as a serialized chain — D1→D2→D3 share the sdk/src + builtin*.rs + row-reader
spine; D4/D5 are parallel-safe leaves behind D3:

- D1 = unpark REQUIREMENT-CLAUSES-RECUT as the chain head. The requirement
  row schema must settle BEFORE the built-in lock is emitted in it: engine
  row readers (drift.rs/document.rs) and sdk/src/contract.ts recut from the
  count?/unique?/membership?/degree? facets to the `clauses?` array
  (10-contracts, set-scope Decision); SEAM_VERSION bumped both sides.
- D2 (blockedBy D1) FIRST-PARTY-MODULE-COMPLETE: @dtmd/temper/claude-code
  carries all four built-in kinds + their floors as exported SDK values,
  every floor clause carrying its cite (URL + retrieved date) migrated from
  packages/*/PACKAGE.md — the citation trail is not residue (10-contracts,
  package-residue paragraph). Acceptance: module content ≡ builtin.rs's
  clauses/guidance/cites, spelled once.
- D3 (blockedBy D2) BUILTIN-LOCK-DERIVED: the embedded default program
  becomes a committed receipt-less lock artifact generated from the
  first-party module's own emit, embedded as data (include_str) and parsed
  at engine startup. builtin.rs + builtin_kind.rs retire as hand-written
  mirrors; kind identity is the compiled row label — resolve_bare/
  qualified_name resolution machinery retires (20-surface); main.rs per-name
  skill/rule dispatch and check::Workspace's skills()/rules() bare-key
  accessors generalize to row-driven ((builtin-workspace-qualified-key)
  closes here — do not file separately).
- D4 (blockedBy D3) BUILTIN-LOCK-FROZEN-LANE: CI re-derives the built-in
  lock from the module and byte-compares against the embedded copy (the
  --frozen discipline aimed at the tool's own std-lib); fail-loud on
  mismatch or absence (50-distribution, fail-loud invariant).
- D5 (blockedBy D3) CURATED-TREES-RETIRE: kinds/ + packages/ trees and
  bundle.rs's CURATED_PACKAGES embeds retire — the plugin ships the skill +
  hook, never clauses (50-distribution, channel 3); delete contract/ if
  still present. The "curated, fence-excluded" asymmetry line in
  open-questions.md dies in the same entry.
