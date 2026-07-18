# Plan state

- Spec derived through: 4adb1fb
- Audited through: f713a08
- Residue swept through: f713a08
- Posture swept through: 4e9d87a — formats done this tick (found
  JSON-MANIFEST-READ-DECODE-CONSOLIDATE and
  TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE, both opened as blockedBy
  chains behind the one currently-open entry each shares a file with).
  This rotation pass has now covered pipeline (4baa5c4, quiet), judges
  (f3980b9), provider (ab4c07d, quiet), foundation (08b5a27), model
  (1c5b0a9), and formats (this tick) — verbs is the one subsystem not
  yet ticked this pass. `git log cfa50fb..HEAD -- src/main.rs
  src/install.rs src/bundle.rs src/lib.rs src/test_support.rs` (cfa50fb
  is verbs' own last sweep) is empty, so verbs is likely quiet, but it
  still needs its tick before this pass can close and the cursor
  advance.
- This tick: POSTURE SWEEP, formats subsystem (frontmatter.rs,
  document.rs, json_manifest.rs, toml_document.rs, per
  architecture.md's codemap: "external format mechanics, implemented
  once"). Read every formats file end to end (1590 lines total).
  Found: (1) `json_manifest.rs`'s `Manifest::read` (331-344) and
  `DocumentMember::read` (145-155) each independently run the identical
  `fs::read` → `JsonManifestError::Io` → `String::from_utf8` →
  `JsonManifestError::NotUtf8` sequence before calling their own
  `parse` — byte-for-byte identical, grep-verified the only two sites
  building a string this way against `JsonManifestError`. Filed
  JSON-MANIFEST-READ-DECODE-CONSOLIDATE (factor a shared
  `read_to_string` helper), blockedBy
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH, the one currently-open
  entry sharing json_manifest.rs. (2) `toml_document.rs`'s `pub fn
  parse` (53-99) has zero caller outside its own `read` in the same
  file — grep-verified zero hits for `toml_document::parse(` anywhere
  in src/, tests/, sdk/; its doc comment claims the same
  soundness-boundary rationale `json_manifest::DocumentMember::parse`
  earns via a real test caller (`tests/json_document_format.rs:193`),
  but no test or production site ever calls this file's `parse`
  directly — `install.rs`'s guard validates only collection-address
  manifests, never a toml-document's pending content. Filed
  TOML-DOCUMENT-PARSE-ZERO-CONSUMER-PRUNE (narrow to private),
  blockedBy the same DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  the one currently-open entry sharing toml_document.rs. Checked and
  found clean beyond these two and the already-queued
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH (json_manifest.rs's
  `DocumentMember::parse` / toml_document.rs's `parse` non-exhaustive
  `UnitShape` match): every enum match in these four files is otherwise
  exhaustive (`value_to_json` over `toml_edit::Value`'s 7 variants,
  `item_to_json` over `toml_edit::Item`'s 4), every other `pub`/
  `pub(crate)` item (`fold_file_id`, `closing_delimiter`, `item_to_json`,
  `Satisfies::new`, `write_manifest`, `write_document`, `to_unit` on
  both member shapes, `Manifest::read_kind`) has a real external
  caller (grep-verified), and no error-enum variant in
  `FrontmatterError`/`JsonManifestError`/`TomlDocumentError` is
  unreachable. `document.rs` bundling `Satisfies` (a cross-cutting
  model concept) with `item_to_json`/`value_to_json` (a formats-only
  TOML→JSON helper) reads as two jobs in one small module, but with no
  duplication or defect risk behind it — noted, not filed; too thin a
  case to force a cohesion split against the "purely mechanical"
  routing bar.
- Queue: 29 pending, 6 pickable OPEN (unchanged set:
  BUILTIN-KIND-DEFINITION-RESULT-COLLAPSE,
  DOCUMENT-IDENTITY-UNIT-SHAPE-EXHAUSTIVE-MATCH,
  ENGINE-JUDGE-SELECTION-EXHAUSTIVE-MATCH, DIAL-IS-EMPTY-ZERO-CONSUMER-PRUNE,
  CHECK-ANNOUNCEMENT-HEADING-ZERO-CONSUMER-PRUNE, DRIFT-LOCK-ROW-WALK-
  CONSOLIDATION — pairwise disjoint on files, unchanged), 19 chained
  blockedBy (17 prior + the 2 filed this tick, both verified disjoint
  from every other currently-open entry's files), 4 parked on human
  action (IMPORT-HOP-CAP-CITE, PACKAGING-CHANNELS-REMAINDER,
  GRAPH-ENGINE-GLOB-EXTRACTOR-CONSOLIDATE, NORMALIZE-PATH-SUBSYSTEM-
  PLACEMENT — reasons unchanged, untouched this window).
  Open forks: (multi-harness-projection), (lazy-grounds) unchanged.
  Refactor captures: none live. Inbox empty.

Plan continues: yes — verbs, the one subsystem this rotation pass
has not yet ticked, is live next tick (likely a quiet verdict, which
would close the pass and advance the cursor).
