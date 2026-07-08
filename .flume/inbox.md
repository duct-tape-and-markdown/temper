<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Field report, three findings (observed at 4b0bb0f), all verified against
  disk before filing:

  **T14 — a kind rename with an unchanged locus deletes the files it still
  governs, reporting them "unchanged" while doing it.** Confirmed:
  `reap_or_report_orphan` (`src/drift.rs:587`) keys reap purely off the
  *prior* lock's `ProvenanceRow.source_path`/`emit_hash` and the current
  on-disk hash at that path — it never cross-checks whether a *live* member
  in the same run also resolves to that exact path. Renaming a custom
  kind's `name` with its locus untouched is indistinguishable from deleting
  the old kind: the renamed kind's fresh "unchanged" write and the old
  kind's stale provenance row both resolve to the same path, and the old
  row's reap pass deletes what the new row just wrote. Repro: kinds `A`/`B`,
  identical locus/members; declare under `A`, emit; rename to `B` with no
  other change; emit again — `B` logs "unchanged", the reap pass for `A`'s
  now-orphaned row deletes the same path immediately after.

  **T15 — check cannot validate requirement satisfaction on any
  SDK-composed harness, full stop.** Confirmed via full causal trace: every
  `require`d requirement reports `requirement.unfilled` even when the
  lock's `[[declaration.satisfies]]` rows are correct, because
  `resolve_kind_units` (`src/main.rs:846`) populates a live `Unit.satisfies`
  from exactly one source — `surface_overlay` (`src/main.rs:822`), which
  reads a per-member document at
  `<workspace>/<kind's surface subdir>/<id>/<KIND>.md` and returns `None`
  if that document was never itself projected. `roster::is_satisfier`
  (`roster.rs:43`) and `coverage.rs:92,120` gate purely off
  `Features.satisfies`, itself only ever set from that same graft
  (`builtin_kind::features`, forwarding `unit.satisfies` with no other
  source). `declarations.satisfies` — the lock's own, correct rows — is
  read in exactly one other place in the entire binary
  (`main.rs:776`, `.is_empty()`, an incoherence boolean) and never as
  gating data. Confirmed no production writer of the overlay document
  exists: every function that creates one (`write_surface`,
  `write_surface_skill`, `write_surface_rule`, in `kind.rs`/`check.rs`) is
  `#[cfg(test)]`-only — a holdover from the pre-0016 own-path/shallow-lift
  era with nothing shipping to replace it. Corroborating: temper's own
  re-dogfooded `.temper/harness.ts` uses the flat module layout and
  declares no `require` block at all, so nothing in the corpus — temper's
  own included — currently exercises a real requirement being gated for
  real.

  **T16 — a custom kind whose name collides with a built-in's silently
  loses its members, no diagnostic.** Confirmed the mechanism: kind-member
  loading merges into a single `BTreeMap<String, _>` keyed by bare `name`
  via a plain `.insert()` with no pre-existing-key check
  (`Workspace::load_kinds`, `check.rs:83`) — its own doc comment
  acknowledges the exact failure mode ("Two kinds sharing a bare name would
  silently overwrite each other's entry here") but assumes it away ("the
  embedded built-in set is guaranteed unique, so a collision is a malformed
  kind set, not a case this loader resolves") — an assumption that holds
  for the built-in-only call path but not once a project's own custom kind
  shares a built-in's name. I did not trace the exact site that renders
  the reported `command (0)` coverage-summary symptom (likely composed
  further up, in `main.rs`'s reporting path) — the unguarded-merge
  mechanism is confirmed, the exact display-layer site is not; worth
  tracing at scoping time rather than re-asserting my guess.

  Priority note (mine, not the reporter's): T15 looks like the highest
  ceiling of the three — it means `require`/`satisfies`, the mechanism a
  whole requirement kind exists to assert, cannot be validated on any
  harness built the current (whole-conversion, post-0016) way. Worth
  scoping first regardless of filing order.
