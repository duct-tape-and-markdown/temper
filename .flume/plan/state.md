# Plan state

- Spec derived through: cd7135b
- Audited through: 99337b8
- Residue swept through: d3c2805
- This tick: Ship audit (job 3). Two commits past a112dbe touched
  src/tests/sdk: 2f87229 (SDK-VERSION-LOCKSTEP) and 90aa57b
  (RETIRE-DEAD-DECLARED-SURFACE); 6df1b76 (CI-DOCUMENTED-TWO-LINE-JOB) and
  99337b8 (the ship commit dropping all three from pending.json) touched no
  src/tests/sdk paths but were read too. Verified on disk, not the log alone:
  install.rs derives `sdk_version_range()` from `include_str!` of
  sdk/package.json (now 0.0.5), no hand-typed literal remains; grep confirms
  `import_hash` and `Template.leaves`/`collections` are gone from src/tests;
  temper.yml runs `check` + `emit --frozen` + `git diff --exit-code`, no
  `import` verb. cargo test: 225+ passed, 0 failed across the suite. All
  three shipped entries' acceptance bars hold — pending.json already reflects
  their removal (99337b8), nothing to drop this tick. Re-tested the stale
  blockedBy gate this triggers: INSTALL-WHOLE-CONVERSION was blockedBy
  SDK-VERSION-LOCKSTEP for file disjointness on src/install.rs and
  tests/install.rs, not a technical dependency; that blocker shipped, so
  promoted its gate to open — its cited symbols (member_module_source,
  description_trigger_value, scaffold) still match current install.rs
  exactly, premise unchanged since e920fa8 scoping. RETIRE-OWN-PATH-MACHINERY
  stays blockedBy INSTALL-WHOLE-CONVERSION (a real dependency: own_path is
  still live). PACKAGING-CHANNELS's parked reason re-verified true (no
  release.yml, root package.json still the private flume manifest, no
  marketplace/signing creds); refreshed its stale "0.0.4" mention to 0.0.5.
- Queue: 2 (was 3) — INSTALL-WHOLE-CONVERSION (open, pickable) →
  RETIRE-OWN-PATH-MACHINERY (blockedBy it); PACKAGING-CHANNELS (parked).

Plan continues: yes — residue sweep (job 4) is next; Residue swept through
(d3c2805) trails HEAD (99337b8) and jobs 1-2 (inbox, spec delta) are quiet.
