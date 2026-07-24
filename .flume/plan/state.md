# Plan state

- Spec derived through: aee005d — unchanged this tick.
- Audited through: 4d9be4e — unchanged this tick.
- Residue swept through: 4d9be4e — unchanged this tick.
- Posture swept through: mid-rotation, at src/graph.rs — src/hash.rs next
  in the c9d11d5 rotation's frontier, untouched this tick.
- This tick: INBOX. Drained item 11 (CI Node 20 deprecation). Re-verified
  its pre-attached disposition rather than transcribing it: the
  "outside build's writablePaths" claim is false (`.github/**` is in
  `chain.ts`'s BUILD_WRITABLE_PATHS; 6df1b76 is a `build:` commit that
  already edited temper.yml). Corrected routing, same accepted-debt
  outcome for a narrower reason — release.yml is human-only by
  release.md's own path scope + unbroken chore/fix(release) precedent;
  temper.yml is fence-eligible but the bump has no specs/-derived `per`
  to cite, so it stays debt too. See commit body for the DATUMs and
  verified target majors. Friction filed on the misleading chain.ts
  comment that caused the mis-disposition.
- Queue: 2 pending — 0 open, 1 parked, 1 deferred. Open forks: 2, unchanged.
  Friction: 1 (filed this tick). Amendments: 0. Inbox: 0.

Plan continues: yes — post-ship reconciliation over 4d9be4e..HEAD
(bf4b5cd, 8272023, afc495e, and this tick's own drain commit) is next
tick's job.
