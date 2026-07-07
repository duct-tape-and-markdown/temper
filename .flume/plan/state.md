# Plan state

- **Phase:** reconcile after COMMENT-VERB-CITATION-SWEEP shipped (6a0878c).
  Spec delta empty (no `specs/` commits since the last plan tick, 4b24d6a);
  inbox empty.
- **Residue sweep this tick:** the manifest/retired-verb noun sweeps have
  **bottomed out** — remaining `manifest` uses are legitimate (real npm/plugin
  JSON manifests) or test-fixture labels; `temper.toml`, `activation`,
  `altitude`/`rung`/`carriage`/`gradient`/`ladder` all clean. One genuine tail
  the last sweep missed: three **stale** spec-heading quotes (headings absent
  from the corpus) — `drift.rs` module doc (`re-add` heading it fixed the body
  cite for but not the header) + two `SDK pins its engine version` cites, and
  `document.rs` module doc. Filed as RETIRE-STALE-HEADING-QUOTES.
- **New fork surfaced:** `(place-three-state-retire)` — `drift.rs::place`
  implements the three-state merge the drift-routing Decision *rejects*; all
  four real callers pass `None` (Conflicted path dead outside one test); needs
  John (does the seam collapse now that `install` shipped?).
- **Last shipped (6a0878c):** COMMENT-VERB-CITATION-SWEEP (retired-verb +
  stale-citation doc-comment sweep across seven files).
- **Queue — 2 entries:** RETIRE-STALE-HEADING-QUOTES (open, comment-only,
  `drift.rs` + `document.rs` — disjoint from PACKAGING) and PACKAGING-CHANNELS
  (parked: no `release.yml`, root `package.json` still the private flume
  manifest, needs human release creds + the engine-binary workflow — nothing
  moved this window).
- **What's next:** build drains the stale-citation fix. Beyond it the queue is
  human-gated — PACKAGING parked, all product forks await John (nearest engine
  work `(json-projection-format)` unblocked, plus the new
  `(place-three-state-retire)`). Observed, release-owned: `src/install.rs`
  still pins SDK `^0.0.2` while `@dtmd/temper` 0.0.3 is published — a bump
  belonging to PACKAGING-CHANNELS, not this sweep.

Plan continues: no — queue reconciled (one pickable entry filed off genuine
stale-citation residue, one fork surfaced), inbox empty, delta empty. Building
drains it.
