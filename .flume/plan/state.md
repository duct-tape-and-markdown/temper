# Plan state

- Spec derived through: cd7135b
- Audited through: a112dbe
- Residue swept through: d3c2805
- This tick: Quiet closing pass (job 5). Re-verified all four inputs: inbox
  empty, spec-delta empty (no specs/ commits past cd7135b), no src/tests/sdk
  commits past a112dbe or past d3c2805 (only the prior plan: commit landed —
  .flume/-only), so jobs 1-4 all stay quiet. Re-checked every queue gate
  reason on disk: temper.yml still runs the dead `import` verb (CI-DOCUMENTED-
  TWO-LINE-JOB's gate holds); SDK_VERSION_RANGE is still the hand-typed
  `"^0.0.4"` literal and sdk/package.json is still 0.0.4, unbumped
  (SDK-VERSION-LOCKSTEP's gate holds, so its two blockedBy dependents still
  correctly block); Template.leaves/collections and document.rs's
  `import_hash` fallback are both still live (RETIRE-DEAD-DECLARED-SURFACE's
  gate holds); PACKAGING-CHANNELS's parked reason re-verified — no
  release.yml, root package.json is still the private flume manifest, no
  marketplace/signing creds. Queue disjointness re-checked: the three `open`
  entries (CI-DOCUMENTED-TWO-LINE-JOB, SDK-VERSION-LOCKSTEP,
  RETIRE-DEAD-DECLARED-SURFACE) touch disjoint file sets; the two chained
  entries share files with SDK-VERSION-LOCKSTEP but are serialized via
  blockedBy, never concurrently open. No entries rewritten — nothing moved.
- Queue: 6 — CI-DOCUMENTED-TWO-LINE-JOB, SDK-VERSION-LOCKSTEP,
  RETIRE-DEAD-DECLARED-SURFACE (open, pickable, disjoint); INSTALL-WHOLE-
  CONVERSION → RETIRE-OWN-PATH-MACHINERY (chained behind SDK-VERSION-
  LOCKSTEP); PACKAGING-CHANNELS (parked).

Plan continues: no — all inputs current, queue disjoint, every gate reason
re-verified true; build takes over the three pickable entries.
