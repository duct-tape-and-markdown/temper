# Plan state

- **Phase:** reconcile. HEAD 1b9f33a. Inbox empty.
- **Last shipped:** EXTRACTION-VOCAB-GAPS — the nested key-path `Primitive::Field`
  fix (`build` d3686b2 / `chore` 1b9f33a; verified on disk: `resolve_key_path`
  walks the dotted key-path, src/kind.rs:839). Build removed it from the queue.
- **This tick — reconcile only:** verified the four remaining entries against disk
  and spec; all four stay **parked**. AGENT-KIND + COMMAND-KIND await their
  human-authored `kinds/claude-code/{agent,command}/KIND.md` + packages (absent on
  disk — no `.claude/agents` or `.claude/commands` dir either). SETTINGS-KIND stays
  parked + `dependsOnForks (json-projection-format)`; refreshed its notes to record
  that its key-path dependency has now shipped — only the `json` format harvest +
  shared adapter remain. PACKAGING-CHANNELS still parked (human release creds).
  Updated the `(json-projection-format)` fork the same way. No new gap fileable:
  the corpus↔code deltas all rest on human curation or an unresolved fork.
- **Pickable now:** none — the whole queue is parked pending human action.
- **What's next (human, not a build tick):** curate the agent/command KIND.md +
  PACKAGE.md (un-parks AGENT-KIND/COMMAND-KIND, serialize on shared builtin.rs/
  kind.rs); harvest `json` into 15-kinds.md (un-parks SETTINGS-KIND + the shared
  json adapter entry); settle `(hook-kind-locus)`. The dogfood's SDK/TS-primary
  migration remains the ledger's ceremony.

Plan continues: no — the queue is reconciled and the inbox is empty; nothing is
pickable (all four entries parked on human action), so there is no build work to
hand off and no further plan work this turn.
