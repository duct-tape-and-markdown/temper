# Plan state

- **Phase:** reconcile + inbox drain. HEAD 0496270.
- **Last shipped (trunk):** law 8 landed in the corpus (750dbc8) and the dogfood
  `.temper/` was reconciled by hand (0496270, references dropped from the spec
  KIND.md); `contracts/` retired by hand earlier (b5f0786). CONTRACTS-RETIRE is
  gone from the queue (shipped) — verified: `contracts/` does not exist on disk.
- **This tick:** drained the inbox (2 notes) into 3 new pending entries, verified
  on disk. (1) REFERENCES-RETIRE — `Primitive::References` + `backtick_filename_refs`
  + `is_filename_reference` + `strip_suffix` still ship in `src/kind.rs` (0496270
  only touched the dogfood); law 8 retires them. (2) REDD-CUSTOM-KINDS — `drift.rs`
  `diff`/`re_add` hardwire skills+rules only, so custom-kind (spec) members never
  reconcile back (the dogfood defect). (3) MEMBER-PUBLISHED-REQUIREMENTS — member
  headers carry only `[satisfies.*]` today, not `[requirement.*]`; coverage reads
  `layer.requirements()` alone. Updated `(reference-id-normalization)` →
  SUPERSEDED. Accepted debt: the specs/ class placement reshuffle is a human
  ceremony after the engine ships (not a build entry).
- **Pickable now (2 `open`, disjoint):** REFERENCES-RETIRE (kind.rs +
  reference_resolution.rs) and REDD-CUSTOM-KINDS (drift.rs + main.rs + drift
  tests) — no shared file. Serialized: MEMBER-PUBLISHED-REQUIREMENTS blockedBy
  REDD-CUSTOM-KINDS (both edit main.rs). Deferred: AGENT-KIND (priority). Parked:
  PACKAGING-CHANNELS (release creds).

Plan continues: no — queue reconciled, inbox drained, two disjoint `open` entries
are pickable; building drains them.
