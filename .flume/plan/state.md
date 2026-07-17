# Plan state

- Spec derived through: ae9810f
- Audited through: 9409a6c
- Residue swept through: 9409a6c
- This tick: SPEC DELTA — routed 0032 (`ae9810f`, local is a locus, not a
  layer) whole; the cursor advances to it alone. 0033 (`f1d97e4`) is next
  tick's job, untouched. **0032's Consequences, bullet by bullet.** (1)
  `representation.md` ("locus"), `pipeline.md` ("Layers", "The lock"),
  `contract.md` ("clause") carry the widenings — moot, read in the diff: all
  three landed in `ae9810f` itself. (2) 0030 gains a dated amendment note —
  moot, `specs/decisions/0030-*.md` carries "Amended — 2026-07-16 (0032)". (3)
  The `(layer-delivery-format)` fork resolves whole and its record deletes —
  moot: `ae9810f` deleted 52 lines from `open-questions.md`; `grep` finds
  neither it nor `(clause-vocabulary-holds)`. (4) "Plan derives the entries:
  the locus class, the label spelling, the dial kind and its TOML format
  mechanics, the `--layer` lock join, the announcement line" — five entries
  filed, one per named derivation: LOCAL-LOCUS-COMMITMENT-CLASS,
  CLAUSE-LABEL-IS-AN-ADDRESS, CHECK-JOINS-INVOCATION-LOCKS, DIAL-KIND,
  CHECK-ANNOUNCES-THE-LOCK-FAMILY. (5) `settings.local.json` is "the first
  **candidate** local-locus layout kind" — registered as
  `(settings-local-kind)`, not promoted: a candidate is not a requirement (the
  `(plugin-author-dogfood)` precedent).
  **Two premises of 0032 failed on disk, and neither is papered over.** The
  label: 0032 says "the lock already writes one per row and refuses
  collisions" — it does not. `ClauseRow` (`src/drift.rs`:2548) has no label
  column; the printed id is `Selector::rule` (`src/engine.rs`:348), the bare
  predicate key, not unique per row; the two shipped collision refusals
  (`src/main.rs`:1713/1787) guard kind names and governs globs, not clause
  rows. The corpus has claimed the label since the founding (`e842a32`) and no
  entry ever built it — so CLAUSE-LABEL-IS-AN-ADDRESS is a column plus a
  grammar plus a refusal, scoped to that; the intent is unambiguous, so no
  fork. The dial: `contract.md` calls it a local-locus **TOML** document while
  `representation.md` declares a local locus layout-only, and a layout is
  markdown's heading tree — two ratified sentences of one commit that cannot
  both hold. Registered `(local-locus-toml-face)`; DIAL-KIND waits on it.
- Queue: 7 entries, **1 pickable** — LOCAL-LOCUS-COMMITMENT-CLASS is `open`;
  the other four 0032 derivations serialize behind it as one `blockedBy` chain
  (every one edits `src/main.rs`, so none may be `open` beside it), with
  DIAL-KIND additionally fork-gated. IMPORT-HOP-CAP-CITE and
  PACKAGING-CHANNELS-REMAINDER stay parked on human acts, both re-tested on
  disk at HEAD and holding.

Plan continues: yes — the spec delta is still live: 0033 (`f1d97e4`, a hold is
named debt) is un-routed, and its four widenings plus the schema-oracle test
are next tick's derivation.
