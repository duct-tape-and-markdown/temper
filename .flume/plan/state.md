# Plan state

- Spec derived through: a53eee4
- Audited through: f000b97
- Residue swept through: a3f9f1f
- This tick: Residue sweep (job 4), a4bb450..a3f9f1f — one src/tests change
  in range, 5ee0b6d (RETIRE-FOLD-MEMBERS). Swept it clean: `rg` confirms
  `fold_members`/`parse_embedded_info`/`parse_embedded_member` have zero
  live callers (one doc-comment historical mention in
  tests/nested_member.rs, already accepted); `Primitive::Fenced` still an
  ordinary primitive with no orphaned callers; the new
  `extract::nested_members_from_rows` / `embedded_member_from_row` /
  `builtin_kind::host_address` read path is the sole builder of
  `EmbeddedMember` in src/ (test-fixture literals in
  tests/display_rule.rs, tests/read_verbs.rs, src/read.rs are ordinary
  test data, not a second implementation) — no residue, no duplicate
  matcher. `cargo check --tests` compiles clean. Re-verified the two
  standing open-questions debts (session_start.rs `+++` fixtures,
  builtins.ts stale PACKAGE.md citations, tests/coverage.rs stale
  `effective_governs` comment): none of their three files touched by
  5ee0b6d; restamped all three to a3f9f1f in open-questions.md.
- Queue: PACKAGING-CHANNELS (parked, unchanged) — the only pending entry.

Plan continues: yes — quiet closing pass (job 5): every input above is
current (inbox empty, spec delta empty, ship audit and residue sweep both
at HEAD a3f9f1f).
