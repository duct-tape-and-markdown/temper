# Plan state

- **Phase:** derived-lock chain + first comment sweep **shipped**; this tick
  files the two residue tails that survived them. Inbox empty; spec delta empty
  (no `specs/` commits since 4ed47a0 — the recent ticks touched only `.flume/`).
- **Last shipped (f27bbf4):** COMMENT-STOCK-SWEEP — cut false tree/edge comments
  in builtin_kind.rs, bundle.rs, read.rs, builtin.rs (verified on disk).
- **This tick:** the residue sweep ran despite the empty delta and found two
  un-derived tails of the package-noun dissolution. (1) The live hand-TOML
  clause-table parser (`Contract::parse` + 9 helpers, contract.rs) is reachable
  **only from tests** — the live gate builds every `Contract` from lock
  `ClauseRow`s via `compose::effective`; the corpus is emphatic ("no clause
  document, no TOML clause table"). Filed RETIRE-TOML-CONTRACT-PARSER (open).
  (2) False package-noun comment narration + dead section-title cites
  (`Packages`, `Templates`) survive in 5 files the first sweep did not cover.
  Filed PKG-NOUN-COMMENT-SWEEP (open). The two edit disjoint file sets
  (contract.rs+2 tests vs. check.rs/kind.rs/graph.rs/document.rs/builtin.rs) —
  parallel-safe.
- **Queue — 3 entries, 2 open:** RETIRE-TOML-CONTRACT-PARSER + PKG-NOUN-COMMENT-SWEEP
  (both open, disjoint). PACKAGING-CHANNELS parked (release creds + engine-binary
  workflow + USPTO — all human, none shipped this window).
- **What's next:** build fans out the two open entries. Beyond them: the
  `activation` → `registration` symbol rename (retired vocabulary, live code —
  noted in open-questions, unfiled to avoid overlap), then the next engine wave
  `(json-projection-format)` (open fork, awaits John on the JSON adapter /
  `layout`-fact spelling).

Plan continues: no — queue reconciled (two disjoint pickable open entries filed
off the residue sweep), inbox empty, delta empty. Building drains it.
