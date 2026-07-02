# Plan state

- **Phase:** reconcile + inbox drain. HEAD 9badebe.
- **Last shipped (trunk):** REFERENCES-RETIRE landed (87da879 build, 9badebe
  flume) — verified: no `References`/`strip_suffix`/`backtick_filename_refs` in
  `src/kind.rs`. Dropped from the queue (shipped).
- **This tick:** reconciled the 4 remaining entries against the corpus + disk.
  REDD-CUSTOM-KINDS confirmed a real gap (`drift.rs` still hardwires
  `DriftEntry.kind: &'static str` + `discover_skill_dirs`/`discover_rule_files`;
  `main.rs` Diff/ReAdd arms only `Workspace::load`) — **rewrote** it to add the
  `src/import.rs` edit (its reused helpers `discover_kind_units`/
  `import_custom_unit` are private today and need a `pub(crate)` bump). Verified
  MEMBER-PUBLISHED-REQUIREMENTS still a gap (`document.rs` parses `Satisfies`/
  `EdgeClause` only, no `[requirement.*]`; `main.rs` feeds `layer.requirements()`
  alone). AGENT-KIND/PACKAGING-CHANNELS unchanged (deferred/parked, accurate).
  Inbox empty; nothing to drain.
- **Pickable now (1 `open`):** REDD-CUSTOM-KINDS. Serialized behind it:
  MEMBER-PUBLISHED-REQUIREMENTS (blockedBy — both edit `src/main.rs`). Deferred:
  AGENT-KIND (priority). Parked: PACKAGING-CHANNELS (release creds).

Plan continues: no — queue reconciled, inbox empty, one `open` entry pickable;
building drains it.
