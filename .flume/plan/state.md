# Plan state

- Spec derived through: 39a4833
- Audited through: ff7da32
- Residue swept through: ff7da32
- This tick: RECONCILE the 525111a..HEAD ship window (build 8eb39fb shipped
  MENTION-ROUTE-RESOLVE-AT-CHECK). Audit: verified on disk that `check` now
  owns the deferred-mention verdict — `graph::route_mentions` (949) resolves
  each mention against the discovered corpus and fires `graph.route` on a
  dangle, wired beside `graph::check` at main.rs:825; `why` route-resolves the
  same split via `graph::partition_mentions` (930) so read never disagrees
  (read.rs:298-440); the now-false "mentions lift off the lock at emit" docs on
  `resolved_mention_edges` were corrected in place. `cargo test --test graph
  --test read_verbs` green (incl. new `mention_narration::*`). Entry already
  removed from pending by ff7da32. Sweep (code vs corpus, same window): the one
  rider whose file this window opened is `src/read.rs`'s retired-CLI-verb strand
  doc comments — 8eb39fb opened read.rs's `why` region but left them as
  unchanged context (reconciliation-not-opening), so undischarged, shifted +25
  below line 270 (470/608/745/1147 → 495/633/770/1172); rider updated. graph.rs
  cite lines (61/689, "narration cites lag") sit far above the 861+ hunks —
  unshifted, untouched, verbatim. No new residue: the docs the entry named as
  false were fixed by the build.
- Queue: 0 pickable. PACKAGING-CHANNELS-REMAINDER parked (human release
  actions: John's Apple notarizing + the v0.1 lockstep tag) — condition
  re-tested, still unmet, validly parked.

Plan continues: no — window reconciled, both cursors advanced to ff7da32; no
spec delta (39a4833 fully derived), inbox empty, no unreconciled src/tests/sdk
commit past ff7da32. Only the parked PACKAGING entry remains, so no pickable
work: the loop hibernates. NB the SessionStart reporter still shows the
`.temper` dogfood gate red (friction-capture-procedure, pending-entry-discipline
unfilled) — harness territory, a `chore(harness)` fix outside plan's writable
paths.
