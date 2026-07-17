# Plan state

- Spec derived through: b8396d4
- Audited through: 14c5de4
- Residue swept through: 14c5de4
- This tick: POST-SHIP RECONCILIATION — the window `63e5b97..14c5de4` (one
  `build:` commit: 3ed8d2b, `json-document`), audited and swept on disk in one
  tick. **PLUGIN-JSON-DOCUMENT-FORMAT verified shipped and already dropped by
  14c5de4**: `Format::JsonDocument` (`src/kind.rs:511`), the label parse (867),
  the SDK union (`sdk/src/kind.ts:22`), `DocumentMember` + `write_document`
  (`src/json_manifest.rs:112/451`), `tests/json_document_format.rs`. **The
  stale narrations that entry named are discharged**: `Format`'s doc no longer
  reads "first and only harvested entry" and `NamedField` (526) now says the id
  is read from "whichever surface the kind's `Format` carries its fields on",
  not "frontmatter field". (`src/kind.rs:1138`'s "sole harvested member is
  `AtImport`" is `DirectiveSyntax`, a different closed vocabulary, accurate at
  HEAD — not residue.) **Gate re-tested and opened:** PLUGIN-MANIFEST-KIND's
  `blockedBy PLUGIN-JSON-DOCUMENT-FORMAT` is discharged → `open`. **The
  sweep's find, which the audit did not predict:** 3ed8d2b made `format`
  load-bearing on the *read* face only. `project_bytes` (`src/drift.rs:1723`)
  renders a `---` frontmatter block over every file member unconditionally —
  `Projection` (562) carries no format and nothing in the emit path reads one —
  so `write_document` ships with `tests/` as its only consumer, and a
  `json-document` member (declarable from the SDK today) projects silently
  wrong bytes at its `.json` path. That contradicts
  `specs/model/representation.md` ("kind": a file-locus kind's format derives
  *two* one-way faces — the canonical writer for projections, the lenient
  reader for sources) and invariant 6. No queued entry carried it: the two kind
  entries declare kinds with zero members here, and BUNDLE-EMIT-THROUGH-KINDS
  consumes the dispatch rather than building it. Filed as
  EMIT-JSON-DOCUMENT-WRITE-FACE, disjoint from every open entry. **Cites
  re-derived:** `governs_collision_diagnostics` 1801 → **1808**
  (PLUGIN-MANIFEST-KIND); `builtin_lock.rs` "generated data" 17-19 → **16-18**.
  Verified unmoved: `all_kinds()` 315, the `kinds.len()` assert **9 at 2093**
  (so the two kinds land it at 10 then 11 — the corpus's "Ten kinds ship"
  counts a different set and is not that assert's check), `bundle.rs`
  178/185/191, every `src/graph.rs` cite. All three accepted-debt records
  re-verified exact and unmoved (`tests/session_start.rs:121/140`,
  `sdk/src/prose.ts`'s ten lines, `Cargo.toml:42-45`) — none has a carrier in
  the queue. The "format implementations are engine code" asymmetry restamped:
  its parenthetical named only the frontmatter adapter, and the inventory now
  holds two. Spec cursor copied forward verbatim: this tick derived nothing.
- Queue: 6 entries — 2 pickable and file-disjoint (EMIT-JSON-DOCUMENT-WRITE-FACE
  in `src/drift.rs`+`tests/json_document_format.rs`; PLUGIN-MANIFEST-KIND in
  `src/builtin_kind.rs`+`sdk/`+`tests/`), 2 serialized behind the kinds on
  shared files, 2 parked on human acts. Both parks re-tested on disk and both
  hold: IMPORT-HOP-CAP-CITE (nothing ruled the hop semantics; the window
  touched no import mechanics, `MAX_IMPORT_HOPS` still 5 at 65 under a cite
  claiming five) and PACKAGING-CHANNELS-REMAINDER (four era tags and no version
  tag, crate 0.1.0 vs npm 0.0.7, release.yml:7-9 verbatim). No file appears in
  two `open` entries.

Plan continues: no — the window is reconciled on both motions, the inbox is
empty, `.flume/refactor/` holds its README alone, and the spec delta is empty
(b8396d4 is still the newest `specs:` commit, routed at 63e5b97). No input
sits below post-ship reconciliation. Build takes over: two entries are
pickable and their cites are fresh as of this tick.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
