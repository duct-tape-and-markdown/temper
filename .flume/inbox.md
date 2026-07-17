<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->
- `(tap-log-format)` ruled 07-17 (interactive, John): option (c)
  sharpened — the log is the engine's own record (the lock / emit-payload
  category), never a member, no declared format; the format vocabulary
  stays three. `pipeline.md` "Telemetry" amended and 0037 amended in
  place (5b6b6f2). Slice [3] rescopes from "local-locus log kind" to
  the record's versioned shape in engine code — JSONL, O_APPEND
  single-line records, a per-line shape version, the reader tolerating
  unknown lines out loud (the 0024 posture by analogy). Slices [1] (tap
  verb) and [4] (field strand) unblock unchanged: the strand's member
  join rides the lock; only the parse is bespoke. observed at 5b6b6f2

- Field defect (centercode, wiring settings.json; session-verified in
  shape): a partially-declared manifest emit **silently deletes discovered
  members and unmodeled segments, and the ledger says nothing**. The corpus
  declared its five `installed-plugin` members before its two hooks; the
  next emit rewrote `.claude/settings.json` as declared — killing the live
  guard + session-start hooks (discovered members the same run's coverage
  counted as `hook (2)`) and the `extraKnownMarketplaces` block — reporting
  `1 emitted, 0 reaped`. The rewrite itself is the model (a represented
  manifest is emit-owned, regenerated whole; declare all of it), but the
  silence is not: pipeline.md's own reap doctrine says nothing authored is
  lost silently ("a drifted ownerless file is a finding, never a silent
  delete") and the `--teardown` cliff guards total file-reap waves, yet a
  manifest write deleting every discovered member of two kinds sails
  through unnamed. Fix direction (derivable — the reap principles extend to
  manifest segments): a reap-ledger line per deleted discovered member, and
  the cliff refusal when the deletion is total. Consumer recovered from
  git; now declares hooks + plugins as members and `extraKnownMarketplaces`
  as settings residue, verified end-to-end in an isolated fixture and on
  the corpus. observed at 4cc3081

- Field defect (centercode, round-2 harvest verifying
  MENTION-REACHABLE-EDGE-UNIFY; session-verified at src/graph.rs:974): the
  unify put field edges in mention-reachable's set, but a citation carried
  by an **embedded posture** still gets no reachability coverage — the
  source endpoint defeats a host-kind clause. `resolved_edges` keys an
  edge's `from` as `(edge.from, source.id)`, so a rule's body-carried
  consult resolves as `from = ("consult", <key>)`, never `("rule", <host>)`;
  `mention_reachable`'s selection ranges over `rule` members and filters on
  the host node, so it never sees the edge the rule's own body carries, and
  the rule's `paths` — the scope gating the whole body, citations included —
  never enters the comparison. Field-proven: `connect-optimus` consults
  `data-access` with two literally-uncovered globs, both endpoints declare
  `paths`, check silent — while `explain data-access` shows the resolved
  edge. This is the **source-side twin** of EMBEDDED-EDGE-DEGREE-SEAM
  (which fixed the target side). Fix direction (derivable — contract.md's
  reference set is "the union of the edges its fields and embedded members
  declare", so host attribution is the corpus's own claim): a placed
  embedded edge's source is its host for host-kind judgment — resolve with
  `from = host`, or walk host→embedded-edge under the host's scope.
  READ-EDGE-UNIFY on the source endpoint. observed at 4cc3081
