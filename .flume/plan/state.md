# Plan state

- Spec derived through: f87cc0c
- Audited through: 5d995a3
- Residue swept through: 5d995a3
- This tick: Quiet closing pass (job 5). Inbox empty, no refactor captures
  live. No `specs/` commits past f87cc0c. No commits touched src/tests/sdk
  past dd38241 — `git log dd38241..HEAD -- src tests sdk` is empty (the two
  intervening commits are both `plan:` bookkeeping), so job 3 has nothing to
  audit and job 4's prior sweep already covers HEAD's source tree; both
  cursors advance to HEAD. Spot-verified the open queue's cited surfaces
  still resolve unchanged: `row_relocates_builtin` (src/main.rs:980-986,
  the `declared.templates.is_empty() || declared.templates ==
  builtin.templates` check intact), `published_requirements` sites
  (extract.rs:328, kind.rs:652, read.rs:1094/1226/1513), `assemble_by_kind`
  called from both `explain` (:438) and `gate` (:662) confirming the
  duplication. PACKAGING-CHANNELS parked reason reconfirmed:
  `.github/workflows/` still holds only `temper.yml`; root `package.json`
  still `"name": "temper-flume-harness"`, `"private": true`. Queue is
  disjoint: TEMPLATES-RELOCATION-COLLISION-REGRESSION (open) touches only
  src/main.rs; RETIRE-DEAD-PUBLISHED-REQUIREMENTS-SURFACE (open) touches
  extract.rs/kind.rs/read.rs — no overlap; MEMORY-ENTERS-REQUIREMENT-CORPUS
  is blockedBy the first so never concurrently open with it.
- Queue: unchanged — TEMPLATES-RELOCATION-COLLISION-REGRESSION open (next);
  MEMORY-ENTERS-REQUIREMENT-CORPUS blockedBy it; RETIRE-DEAD-PUBLISHED-
  REQUIREMENTS-SURFACE open, disjoint; PACKAGING-CHANNELS parked.

Plan continues: no — all inputs current, queue disjoint and pickable, gate
reasons re-verified true; hand off to build.
