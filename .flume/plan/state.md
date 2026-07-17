# Plan state

- Spec derived through: b8396d4
- Audited through: 63e5b97
- Residue swept through: 63e5b97
- This tick: POST-SHIP RECONCILIATION — the window `b3a1636..63e5b97` (two
  `build:` commits: 693f31f, the example's edge targets as sets; 9f22de2,
  `installed-plugin`), audited and swept on disk in one tick. **Both entries
  verified shipped and already dropped from the queue by ce1a690** — no
  pending entry survived its work. `installed-plugin` reads on disk as
  declared: `claude_code_installed_plugin` (`src/builtin_kind.rs:293`) in
  `all_kinds()` (320), `enablement` channel, `enabledPlugins.*` address,
  `tests/installed_plugin_kind.rs` present, and `dead_registration` gained the
  enablement arm gating only the documented `false` (`src/graph.rs:706-723`).
  **The audit's one named debt discharged:** open-questions claimed
  INSTALLED-PLUGIN-KIND would carry `src/read.rs`'s five stale strand doc
  comments; it did — `rg 'temper (why|impact|context|requirements)' src/read.rs`
  is now empty, all five re-worded to "`explain`'s **X** strand"
  (270/495/633/770/1172), leaving `temper explain <target>` at 190 as the one
  verb spelled. Record deleted. **The audit's real find is a defect it
  corrected in the queue, not a ship:** MARKETPLACE-KIND predicted "ten kind
  rows" for `tests/lock_declaration_rows.rs`'s count assert, and disk
  falsifies it — 9f22de2 moved that assert 8 → 9 (2093), so the two queued
  kinds land it at **eleven**, not ten. The corpus's "Ten kinds ship" and the
  engine roster count different sets and neither checks the other:
  `supporting-doc` ships without joining that enumeration, exactly as
  `requirement` does, so the walk always runs one above the corpus's number.
  Both kind entries re-worded to state which count they mean and to read the
  assert rather than the arithmetic. **Line cites re-derived across the queue**
  (the window moved four cited files): `src/kind.rs` — `Format` 488 → 503, its
  doc 484-487 → 496-501, `format_from_label` 836-841 → 858-863, `NamedField`
  508-512 → 526-530; `src/json_manifest.rs` read faces 160/248 → 163/251;
  `sdk/src/kind.ts` `format` doc 142-143 → 144-145 (the `Format` union stays at
  17); `src/builtin_kind.rs` `all_kinds()` 281 → 315 (both kind entries).
  `src/main.rs`'s dispatch (1160) and its comment (1155-1159), and
  `governs_collision_diagnostics` (1801), did not move.
  PLUGIN-JSON-DOCUMENT-FORMAT's central claim re-verified and still true:
  nothing branches on `Format` — four constructions, one label parse, two
  doc-links, one test assert. **Sweep: no fileable gap.** The window's only
  stale narration is `src/kind.rs`'s "first and only harvested entry" and
  `NamedField`'s "frontmatter field", both already named as edits in
  PLUGIN-JSON-DOCUMENT-FORMAT; `src/builtin_lock.toml:34`'s "nine kinds carry
  eight contracts' clauses" is accurate at HEAD, not residue. All three
  accepted-debt records re-verified exact and unmoved
  (`tests/session_start.rs:121/140`, `sdk/src/prose.ts`'s ten lines,
  `Cargo.toml:42-45`) — none has a carrier in the queue. Spec cursor copied
  forward verbatim: this tick derived nothing.
- Queue: 6 entries — 1 pickable (PLUGIN-JSON-DOCUMENT-FORMAT, in
  `src/`+`sdk/`+`tests/`), 3 serialized behind it on shared files
  (`kind.rs`/`builtin_kind.rs`/`builtins.ts`/`bundle.rs`), 2 parked on human
  acts. Both parks re-tested on disk this tick and both hold: IMPORT-HOP-CAP-
  CITE (nothing ruled the hop semantics; 9f22de2 edited `src/graph.rs` only at
  690+, disjoint from every cite, all unmoved — `MAX_IMPORT_HOPS` still 5 at 65
  under a cite claiming five) and PACKAGING-CHANNELS-REMAINDER (four era tags
  and no version tag, crate 0.1.0 vs npm 0.0.7, release.yml:7-9 verbatim).
  No file appears in two `open` entries.

Plan continues: no — the window is reconciled on both motions, the inbox is
empty, `.flume/refactor/` holds its README alone, and the spec delta is empty
(b8396d4 is the newest `specs:` commit, routed at 63e5b97). No input sits
below post-ship reconciliation. Build takes over: PLUGIN-JSON-DOCUMENT-FORMAT
is pickable and its cites are fresh as of this tick.

**One thing for a human, unchanged and not the loop's:** decision 0030 is
still a hole — `specs/decisions/` runs 0023…0029, 0031, and 0030 (`review is
the price of softening`) is orphaned at d6381b4, reverted by this phase's own
`continuation marker is honest` gate firing on a human `specs:` commit.
Recoverable via `git show d6381b4`; John's alone to restore; the misfire is
filed at
`.flume/friction/plan-continuation-gate-reverts-human-specs-commits.md`.
