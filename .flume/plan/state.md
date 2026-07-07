# Plan state

- **Phase:** package-noun dissolution's two residue tails **shipped** (45f8692);
  this tick files the vocabulary residue those tails uncovered but did not
  cover. Inbox empty; spec delta empty (no `specs/` commits since 4ed47a0).
- **Last shipped (45f8692):** RETIRE-TOML-CONTRACT-PARSER + PKG-NOUN-COMMENT-SWEEP
  (verified: the hand-TOML `Contract::parse` is gone; the first sweep's five
  files are clean).
- **This tick:** the residue sweep found the retired `activation` vocabulary
  live across the engine AND a half-done SDK rename — `sdk/src/kind.ts`/
  `builtins.ts` already carry `Registration`, but `declarations.ts` still emits
  the lock column as `activation`, and all of src/ (kind/graph/read/drift/main/
  builtin_kind/builtin_lock.toml) plus the world-edge finding strings still
  speak it. The corpus retired it (`15-kinds.md`: registration generalizes
  activation; no-synonyms, `50-distribution.md`). Filed
  RENAME-ACTIVATION-TO-REGISTRATION (open) — the rename open-questions flagged
  as "dies when the two tails land," now derivable. A second package/altitude
  comment tail survives in the three files neither sweep nor rename touches
  (contract.rs/bundle.rs/extract.rs) — filed PKG-NOUN-COMMENT-SWEEP-II (open),
  disjoint from the rename.
- **Queue — 3 entries, 2 open:** RENAME-ACTIVATION-TO-REGISTRATION (open, sole
  owner of ~14 files incl. sdk/src/declarations.ts) + PKG-NOUN-COMMENT-SWEEP-II
  (open, contract/bundle/extract — disjoint). PACKAGING-CHANNELS parked (release
  creds + engine-binary workflow + USPTO — all human, none shipped this window).
- **What's next:** build fans out the two open entries (the rename runs alone
  over its files; the comment sweep runs alongside). Beyond them: the open
  product forks await John — nearest engine work is `(json-projection-format)`
  (the generic JSON adapter + `layout`-fact spelling) and
  `(edge-representation-unify)` (join→`routes_to` flattening), both needing a
  human decision before they file.

Plan continues: no — queue reconciled (two disjoint pickable open entries filed
off the residue sweep), inbox empty, delta empty. Building drains it.
