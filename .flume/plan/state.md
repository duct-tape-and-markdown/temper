# Plan state

- Spec derived through: 5945405
- Audited through: 01162ee
- Residue swept through: b2afc32
- This tick: ship audit b2afc32→HEAD. LOCK-NESTING-TEMPLATES shipped (0a50f65)
  and verified on disk: `KindFactRow.templates` column (src/drift.rs), lifted
  by `from_kind_fact_row` into inert Template shape (src/kind.rs), `templatesFor`
  SDK emit (sdk/src/declarations.ts); round-trip test green (lock_declaration_
  rows). Already removed from pending by the flume ship chore (01162ee). Its
  downstream blocker cleared → flipped CITE-RETAG blockedBy→open (shared
  drift/kind/declarations no longer contended by any open entry; PACKAGING
  touches only package.json + release.yml). Re-tested PACKAGING park reason on
  disk: root package.json still private `temper-flume-harness`, no
  `.github/workflows/release.yml` (only temper.yml), install.rs still pins
  `^0.0.2` — nothing moved this window, stays parked. Audited cursor → HEAD;
  spec + residue cursors copied forward verbatim.
- Queue: 2 — CITE-RETAG (open, pickable), PACKAGING-CHANNELS (parked: release
  creds + engine-binary workflow).

Plan continues: yes — residue sweep still live (Residue swept through b2afc32
trails HEAD 01162ee; the one intervening code commit is the GENRE-FOLD-residual
closure — conformant, comment/cite staleness rides CITE-RETAG). One job per
tick: ship audit taken, residue sweep next.
