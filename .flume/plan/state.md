# Plan state

- Spec derived through: f67303c
- Audited through: 9223917
- Residue swept through: 9223917
- This tick: INBOX drain of the four fresh notes (the batch above the PR #20
  provenance banner; the reverted 5e353c4 attempted the same four and was
  reverted only for a notes-length overflow, not scope). All four re-verified
  live on disk at HEAD (no src/sdk/tests commit since aba7e47): install.rs:88
  hardcodes `temper check . --reporter session-start`; emit.ts:162-167
  `renderTextBody` rejects any address outside the program-scoped set;
  drift.rs's `reap_or_report_orphan` reaps ownerless projections; main.rs:39
  shares DEFAULT_WORKSPACE between `--into` and the check arg. Routed: check
  half-gate → CHECK-ARG-HALF-GATE (open); `--into` reap wave →
  EMIT-INTO-REROOT-REAP (blockedBy CHECK-ARG — both edit main.rs, disjointness
  rule); composed-mention-can't-target-discovered → fork
  `(composed-mention-discovery-locus)` (SDK/engine boundary gap, no spec-faithful
  fill); 0022 glob-validity → GLOB-VALIDITY-PREDICATE (open, disjoint). Spec
  cursor advanced to f67303c: 0022's own commit routes its work order through
  the inbox, so this note IS the derivation of the spec delta. 0022 Consequences
  checklist: (1) Predicate enum + schema surface → GLOB-VALIDITY-PREDICATE
  (contract.rs); (2) rule & skill default contracts gain the clause, fresh cite
  → same entry (builtins.ts); (3) frozen lock re-derives → same entry
  (builtin_lock.toml); constraint "no author-facing pattern clause" carried in
  the entry description; rejected `tools-must-resolve` recorded do-not-refile.
- Queue: CHECK-ARG-HALF-GATE (open) + GLOB-VALIDITY-PREDICATE (open, disjoint)
  pickable; EMIT-INTO-REROOT-REAP (blockedBy CHECK-ARG); PACKAGING-CHANNELS-
  REMAINDER (parked — John's Apple notarizing + v0.1 tag).

Plan continues: yes — inbox still holds the PR #20 carried block (seven notes,
lines 59+): each a substantial fork/entry with standing-objection framing,
drained in later ticks. Draining all eleven in one commit would corner-cut the
fork records; the four fresh notes are this tick's coherent batch.
