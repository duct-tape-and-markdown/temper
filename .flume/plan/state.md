# Plan state

- **Phase:** reconcile. HEAD 7383907.
- **Last shipped:** BINDING-QUALIFY (build fd4d142, chore ad2d3a9) — the
  `(kind-harness-axis)` build-out is fully landed; kind bindings resolve through
  qualified provider identity.
- **This tick:** drained the inbox (2 human notes) into two new **open** entries —
  CUSTOM-UNIT-REPRESENTATION-CARRY (`import_custom_unit`, import.rs:391, writes a
  provenance-only header with no `carry_representation`, so a re-import silently
  wipes the hand-authored `[requirement.*]`/`[satisfies.*]` trace) and
  READ-VERBS-PUBLISHED-DEMANDS (`read.rs:117` reads the assembly roster only, so
  `requirements` lists 2 of 19 and `why 45-governance` misreports live joins as
  dangling while `check` is green). Both confirmed on disk; file sets disjoint
  (import.rs vs read.rs/main.rs), so both are parallel-safe `open`. The 5 existing
  parked/deferred entries reconciled clean — all cited line numbers verified
  (kind.rs:30/545/588/1171, builtin.rs:37-44), none shipped.
- **Session-start note (accepted, not queued):** the 17 `requirement.dangling`
  findings in the session banner are a **stale installed binary**
  (`~/.cargo/bin/temper` predates the member-published-requirements union) — the
  freshly-built binary's `check` and `session-start` are both clean. Fix is
  operational (`cargo install --path .`), not spec/build work.
- **In flight / pickable:** CUSTOM-UNIT-REPRESENTATION-CARRY and
  READ-VERBS-PUBLISHED-DEMANDS (both `open`). Parked: MEMORY-KIND,
  PACKAGING-CHANNELS, COMMUNITY-DOCS. Deferred: EXTRACTION-VOCAB-GAPS, AGENT-KIND.
- **Next:** build picks the two open dogfood-fix entries. The parked/deferred queue
  still awaits human action; the OPEN strategic forks (edge-representation-unify,
  default-assembly-as-data, eval-capability, multi-harness-projection) stay
  human-to-settle.

Plan continues: no — inbox drained into two disjoint pickable `open` entries, the
existing queue reconciled clean. Building is how the queue drains; hand to build.
