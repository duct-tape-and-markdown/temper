<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at cadabd8e (flume-main's read of 0048/0049, session-verified) —
  two bars for entries already filed. (1) Every 0048 conservation test
  (layout, declarations.ts, graph seams) needs a vacuity pin — `n > 0`
  spans/facts/addresses judged, asserted before the verdict — or the test
  passes over an empty read; add to the acceptance of each conservation
  entry. (2) 0049's ambiguity refusal must fire at every consumer of a
  bare key, not only `graph::resolves` — `explain`'s bare-name path
  (`read.rs` why_impl collapses same-kind matches to the first),
  `embedded_hosts_by_source`, and the SDK member table's bare lookup —
  each names every candidate host; add to
  GRAPH-EDGE-TARGET-HOST-QUALIFIED-ADDRESS / NESTED-MEMBER-DUPLICATE-KEY-
  ADMISSIBILITY / SDK-MEMBER-TABLE-NESTED-EDGE-TARGET acceptance.

- observed at 3571eb18 (build gate-revert 8e4e6b63, entry
  SECTION-CONTAINS-LABEL-MARKER-COLLISION) — the build widened
  `contract::clause_label`'s signature to take the heading and marker, so
  five callers (`compose`, `engine`, `graph`, `roster`, `schema`) and six
  tests constructing clause rows changed by two lines each: 14 files for a
  fix whose entry names three. Before widening files[] to those eleven, cut
  the design smaller: the label is stamped at emit from `row.field`
  (`drift.rs` stamp_clause_label); fold heading and marker into that one
  column at the SDK lowering (`declarations.ts` clauseRow: `field =
  \`${heading}.${marker}\`` for section_contains, the joined section list
  for require_sections) so `clause_label(owner, predicate, field)` and
  every caller stay untouched, and the engine's reader (`contract.rs`
  clause-from-row) splits the column back for the predicate. Then files[]
  is `sdk/src/declarations.ts`, `src/contract.rs`, `src/drift.rs`,
  `tests/lock_declaration_rows.rs`, plus the SDK-compiled regression the
  audit asked for. Blast radius stays "label values re-spell on those two
  predicates' rows only", which the audit already accepted.

- observed at 7c64c1af (cascade-integrations, live run against a real
  block-mode harness; reproduced on THIS harness by the session) — the
  shipped GH #39 fix (de4d5498, GUARD-PATH-MATCH-ABSOLUTE-SINGLE-SEGMENT)
  does not fix the form install wires. `temper guard .` (what
  `install.rs` writes into every settings.json) still exits 0 for an Edit
  to the root `CLAUDE.md` in block mode; `temper guard /abs/root` exits 2.
  Mechanism: `path::relativize_against_root` normalizes the root, and
  `normalize_path(".")` drops CurDir to an empty PathBuf; `strip_prefix("")`
  succeeds on any absolute path, so the untouched absolute path reaches
  `path_matches`'s retained suffix compare and the single-segment rule
  rejects it exactly as before. The regression test passes because
  `tests/common/mod.rs` run_guard hands an absolute tempdir as root —
  nothing exercises `guard .`. Second defect in the same ship: the retained
  suffix heuristic is now a pure false-positive source — under an absolute
  root `<root>/reference/CLAUDE.md`, not a projection, is blocked, and the
  recut test asserts that as desired. After relativization equality is the
  only correct compare. Reopen the entry (its own test encodes the wrong
  contract), files[] `src/path.rs`, `src/install.rs`, `tests/install.rs`,
  `tests/common/mod.rs`: resolve the root by `current_dir().join(root)` (or
  canonicalize) before relativizing; compare relativized path to candidate
  by equality; drop the nested-match assertion; add a test that runs guard
  with `current_dir = root` and `"."` as the argument. GUARD-MANIFEST-WRITE
  is building now and shares `install.rs`/`tests/install.rs` — chain.
  #40 fix (1) verified live in the same run (settings.json Edit blocked).
