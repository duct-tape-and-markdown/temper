# Plan state

- **Phase:** reconcile. Queue reconciled to the corpus; inbox empty; no new gaps.
  HEAD 50c0645; tree clean.
- **Last shipped (trunk):** READ-VERBS + SECTION-CONTAINS-PREDICATE (4a33070) and the
  dual license MIT OR Apache-2.0 (aa52386/50c0645). Verified on disk: `src/read.rs`
  carries `why`/`requirements`; `section_contains` + the `sections` extractor are in
  `kind.rs`/`extract.rs`/`contract.rs`/`engine.rs`; `LICENSE-MIT` + `LICENSE-APACHE`
  present. The engine is now broad against the corpus — drift/apply/re-add, bundle,
  install, schema, reporters, coverage/graph, the roster set-scope predicates
  (count/membership/unique/degree/range), and custom `.temper/kinds/<name>/KIND.md`
  authored kinds all shipped.
- **This tick:** confirmed READ-VERBS/SECTION-CONTAINS/OFFERING-LICENSE shipped (already
  dropped from the queue by their chore commits); verified AGENTS.md still absent, so
  AGENTS-MD stays `open`. Fixed AGENT-KIND's stale note (cited the now-shipped READ-VERBS
  as a then-open main.rs entry). No new src↔spec gap found: the offering community surface
  (README hero, CHANGELOG stub, `.github/{CONTRIBUTING,SECURITY,ISSUE_TEMPLATE}`) is all
  on disk. Inbox empty.
- **Pickable now (1 `open`):** AGENTS-MD (AGENTS.md alone, disjoint from everything).
  Deferred: AGENT-KIND (priority — more built-in kinds is the wrong direction).
  Parked: PACKAGING-CHANNELS (human release creds). Forks: KIND-* + the strategic set
  remain RESOLVED/OPEN decision records with no filed dependents.

Plan continues: no — queue reconciled, inbox empty, AGENTS-MD is pickable `open`;
building drains it.
