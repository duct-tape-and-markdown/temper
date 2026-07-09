# Plan state

- Spec derived through: a53eee4
- Audited through: 6d6ae89
- Residue swept through: a561e70
- This tick: Residue sweep. Only src/tests/sdk-touching commit since 3c6f50b
  is 18d3406 (EMBEDDED-LEAF-TEXT; already fully characterized by the prior
  ship-audit tick). Read its full diff fresh for this sweep rather than
  reusing that characterization: declarations.ts/emit.ts's
  declaredRequirements/declaredAddresses move is a consolidation (one shared
  address set replacing two copies), not new residue; no new duplicate
  matcher/normalizer/encoder introduced. The commit's own gap (a render()
  hook still receiving raw, unresolved leaves — bypassing the
  dangling-mention refusal a render-less kind gets) was filed to
  `.flume/friction/build-embedded-leaf-text-render-hook-gap.md` — confirmed
  present on disk; friction is human-drained per its own README ("humans
  drain it out of band... then DELETE the file"), not a plan input, so left
  untouched. Re-verified all three standing "kept on purpose" debts against
  current disk, independent of the prior tick's diff-based check:
  tests/session_start.rs:128-146 still writes `+++`-format fixtures;
  sdk/src/builtins.ts:9,49,86 still cites the three deleted PACKAGE.md
  files; tests/coverage.rs:336-338's comment still names the pre-rename
  `effective_governs`; sdk/src/kind.ts still carries 8 and sdk/src/contract.ts
  still carries 12 pre-reorg spec-path citation hits. All four unchanged and
  still accurate — refreshed their re-verification stamps to this sweep's
  HEAD (a561e70) in open-questions.md. No new residue class found; no
  pending-entry or open-questions rewrite needed beyond the stamps.
- Queue: PACKAGING-CHANNELS only (still parked on human release creds + the
  engine-binary workflow). No open buildable entries.

Plan continues: yes — every cursor (spec derived, audited, residue swept)
now sits at or past HEAD (a561e70) and the queue is disjoint; the next tick
is the quiet closing pass (job 5).
