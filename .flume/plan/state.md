# Plan state

- Spec derived through: f87cc0c
- Audited through: 2c853af
- Residue swept through: fba4e32
- This tick: Residue sweep (job 4), cursor 5d995a3..fba4e32. No commits in
  range added new residue (3e125da/19a4a5c/2c853af were already reconciled by
  the prior ship-audit tick; fba4e32 is itself a plan commit). Reconfirmed
  both accepted debts live and unchanged: tests/session_start.rs:121-146
  still writes `+++`-format `.temper/kinds/spec/KIND.md` +
  `.temper/packages/spec/PACKAGE.md` fixtures; sdk/src/builtins.ts:308,348,385
  still doc-comment-cite the three deleted `packages/*/PACKAGE.md` files —
  both still ride the next entry that touches their file, per
  open-questions.md. Swept for new residue beyond those two: zero
  TODO/FIXME/XXX/HACK; no unfiled "duplicate implementation" comments (the
  one open gate/explain duplication is already MEMORY-ENTERS-REQUIREMENT-
  CORPUS); specs/builtins.md's five shipped kinds match exactly between
  src/builtin_kind.rs and sdk/src/builtins.ts; no half-built manifest
  machinery beyond the tracked `(manifest-authoring-surface)` fork
  (src/bundle.rs's plugin/marketplace manifest emit is a distinct, already-
  shipped feature, not Reach residue); no stray deprecated/legacy vocabulary.
  Nothing new to file.
- Queue: MEMORY-ENTERS-REQUIREMENT-CORPUS open (next, disjoint — touches
  only src/main.rs); PACKAGING-CHANNELS parked, disjoint (touches
  package.json + a new release.yml).

Plan continues: yes — all four inputs (inbox, spec delta, ship audit,
residue sweep) are now current; job 5 (quiet closing pass) is next.
