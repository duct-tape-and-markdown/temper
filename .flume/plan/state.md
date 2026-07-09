# Plan state

- Spec derived through: f87cc0c
- Audited through: c93eeed
- Residue swept through: e6d0311
- This tick: Residue sweep (job 4). `git log e45d424..HEAD -- src tests sdk`
  showed exactly one code-touching commit since the prior sweep: e5daf1d
  (ships BUILTIN-KIND-TEMPLATES-OVERLAY, already reconciled in the prior ship
  audit at e6d0311). Verified on disk: neither standing accepted debt moved —
  `tests/session_start.rs` still writes the `+++`-format `.temper/kinds/spec/
  KIND.md` + `.temper/packages/spec/PACKAGE.md` fixtures; `sdk/src/
  builtins.ts:308,348,385` still doc-comment-cites the three deleted
  `packages/{rule,memory}.anthropic|memory.agents-md/PACKAGE.md` files —
  e5daf1d touched only src/kind.rs, src/main.rs, tests/lock_declaration_rows.rs,
  none of the two debt files. Found one new residue class: e5daf1d's rename
  of `effective_governs` to `overlay_builtin_kind` (src/main.rs) reached
  every call site and doc comment except `tests/coverage.rs:336-338`, whose
  doc comment still names the retired symbol — per the job's own comment-
  staleness exception this rides the next entry that opens that file rather
  than filing standalone; recorded in open-questions.md's "Kept on purpose"
  for the next sweep to re-check. `rg` for `overlay_templates`/
  `overlay_builtin_kind` confirmed no second implementation (One job, one
  home). pending.json unchanged (PACKAGING-CHANNELS, still parked — re-tested
  last tick, not this one). `cargo check --all-targets` green (see
  cargo-check).
- Queue: PACKAGING-CHANNELS (parked, package.json + a new release.yml) — the
  only pending entry.

Plan continues: yes — quiet closing pass (job 5) is next: inbox empty, spec
delta empty, ship audit and residue sweep cursors both current as of this
tick's HEAD.
