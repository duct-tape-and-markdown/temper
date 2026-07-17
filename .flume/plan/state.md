# Plan state

- Spec derived through: 832f015
- Audited through: 15e5924
- Residue swept through: 15e5924
- This tick: RECONCILE `7629fb0..15e5924` — both motions over 09ef5ea's ship,
  the window's only commit touching `src/`/`tests/`/`sdk/`.
  **Audit: the read face shipped as scoped, and it drains the queue's last
  blocker.** Verified on disk, not off the log: `Format::TomlDocument`
  (`src/kind.rs`:563-568) lifts from `"toml-document"` (975), `read_file_unit`
  dispatches it to `toml_document::read(...).to_unit()` (`src/main.rs`:1276-1277),
  and `local_document_rows`'s match names it beside the two other read faces
  (1433). The ship commit (15e5924) removed the entry.
  **The head entry's gate re-tested and opened.** LOCAL-GOVERNS-OVERRIDES-DISCOVERY
  rested on the toml face alone — its workspace-skip fixture is the dial's own
  `.temper/dial.toml`, which now reads under the shipped face. It is the queue's
  one pickable entry.
  **Cites re-stamped, re-read on disk, never carried.** 09ef5ea's toml module
  shifted `src/drift.rs`'s whole row family ~78 lines: `ClauseRow` 2627→2704,
  `value_type` 2672→2760 (written 3511-3512→3602-3603, read 3553→3647),
  `read_declarations` 3104→3192, `parse_declarations` 3127→3215 — reaching three
  entries. `src/main.rs` moved the admissibility pair 829/868→830/869 and
  `assemble_lock_family` 1381→1385; `src/kind.rs` moved `CustomKind::local`
  728→735 and `local_locus_fault` 743→750. DIAL-KIND's `src/kind.rs` note is
  re-worded off its queued-upstream framing: two of its three 0034 halves are
  now shipped code (bce89b7, 09ef5ea).
  **One finding of my own making.** LOCAL-GOVERNS-OVERRIDES-DISCOVERY's
  `src/import.rs` addresses were wrong at scope time and no sweep had re-read
  them — `git_ignore` 355→354, the `WORKSPACE_DIR` skip 358-365→357 (block
  362-366), the nested-root stop 365-372→367-375. The file was never touched
  since 832f015, so this is scoping error, not drift: an entry's cites are
  re-read on disk even when the window proves the file unmoved.
  **Sweep: clean.** No second implementation — `toml_document.rs` reuses
  `json_manifest::DocumentMember` and `document::item_to_json` rather than
  triplicating the whole-artifact read; only the grammar is new. The exhaustive
  `project_bytes` match closed a real fail-open (a TOML member would have landed
  `---`-fenced bytes at a `.toml` path) and both callers raise their own refusal.
  Nothing filed. Both parks re-tested and hold: the window touches neither
  `src/graph.rs` nor `.github/`. The `src/roster.rs`:470 orphan cite still waits
  for a carrier, unmoved. One open-questions record restamped: the
  format-inventory asymmetry said "the inventory's second entry" and there are
  now three — re-worded, and it names the exhaustive-match proof that keeps
  "deliberate addition" mechanical.
- Queue: 10 entries, **1 pickable** — LOCAL-GOVERNS-OVERRIDES-DISCOVERY. Seven
  chain behind it, serialized on shared files; no entry rests on a fork. Two
  parked.

Plan continues: no — every input is drained. Inbox and refactor captures are
empty, the spec delta is empty (nothing past 832f015), and both reconciliation
cursors now sit at 15e5924 with the window's audit and sweep complete. Build
takes over: LOCAL-GOVERNS-OVERRIDES-DISCOVERY is pickable, and seven entries
queue behind the discovery override 0034's errata ratified.
