# Plan state

- **Phase:** reconcile after RETIRE-MANIFEST-NOUN + REFRESH-STALE-SPEC-CITATIONS
  shipped (ab5eb5d). Spec delta empty (no `specs/` commits since 3481fe1); inbox
  empty. This tick's residue sweep found a further doc-comment tail the two
  shipped sweeps did not target: retired **verbs** used as current behavior
  (`re-add`, `apply`, `init`→install, `read`→explain) and five spec-section
  citations quoting renamed/superseded headings ("The graph scope",
  "Registering a custom kind", the re-emit/patch drift Decision, the
  five-facts heading, the fenced-block heading).
- **Last shipped (ab5eb5d):** RETIRE-MANIFEST-NOUN (manifest noun +
  read-verb diagnostics) and REFRESH-STALE-SPEC-CITATIONS — the noun/citation
  sweeps over the two file sets.
- **Queue — 2 entries:** COMMENT-VERB-CITATION-SWEEP (open, comment-only across
  compose/frontmatter/check/drift/graph/extract/main — one serial commit) and
  PACKAGING-CHANNELS (parked: engine-binary release workflow + human release
  creds + USPTO screen — none moved this window; no `release.yml`, root
  package.json still the private flume manifest).
- **What's next:** build drains the comment sweep. Beyond it the open product
  forks await John — nearest engine work `(json-projection-format)` (unblocked)
  and `(edge-representation-unify)`. Observed, release-owned, NOT filed as a
  sweep: `src/install.rs` pins the SDK `^0.0.2` while `@dtmd/temper` 0.0.3 is
  published — a version bump belonging to PACKAGING-CHANNELS.

Plan continues: no — queue reconciled (one pickable comment sweep filed off the
verb/citation residue), inbox empty, delta empty. Building drains it.
