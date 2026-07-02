# Plan state

- **Phase:** reconcile + inbox drain. HEAD 7333235.
- **Last shipped (trunk):** REDD-CUSTOM-KINDS landed (cb48750 build, 7333235
  flume) — diff/re-add now scan custom kinds so spec edits reconcile. Its
  downstream, MEMBER-PUBLISHED-REQUIREMENTS, unblocks.
- **This tick:** flipped MEMBER-PUBLISHED-REQUIREMENTS `blockedBy
  REDD-CUSTOM-KINDS` → `open` (blocker shipped) and refreshed its stale
  serialization note. Re-verified it is still a real gap: `document.rs` parses
  `[satisfies.*]` only (no `[requirement.*]`); `extract.rs` `Features` carries
  `satisfies` alone; `main.rs` feeds `layer.requirements()` to coverage/roster
  with no member-published union; `compose::Requirement` fields are pub (reuse
  confirmed). AGENT-KIND (deferred/priority), PACKAGING-CHANNELS (parked/creds)
  unchanged — accurate. Inbox empty; open-questions unchanged
  (`(edge-representation-unify)` still the one live OPEN fork, no dependent).
- **Pickable now (1 `open`):** MEMBER-PUBLISHED-REQUIREMENTS — sole open entry,
  parallel-safe. Deferred: AGENT-KIND. Parked: PACKAGING-CHANNELS.

Plan continues: no — queue reconciled, inbox empty, one `open` entry pickable;
building drains it.
