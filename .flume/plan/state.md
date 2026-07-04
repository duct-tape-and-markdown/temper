# Plan state

- **Phase:** reconcile + inbox-drain. HEAD 97adf2e.
- **Last shipped:** both SDK-seam tails (GATE-READS-ASSEMBLY, SDK-PLACEMENT-
  ROUNDTHROUGH; `chore` e1a1fb5) — all four ruled SDK seams closed. No new build
  commit since; the queue was idle pending John's harness-kind ruling.
- **This tick — drained John's 07-04 inbox ruling (harness kinds in scope):**
  revived **EXTRACTION-VOCAB-GAPS** to `open` (the nested key-path `Primitive::
  Field` fix — consumer now named: the settings kind; testable now over nested
  yaml-frontmatter, kind.rs:835 still flat-reads). Revived **AGENT-KIND** and
  filed **COMMAND-KIND**, both `parked` on human curation (their KIND.md/PACKAGE.md
  are fence-excluded ratification territory). Filed **SETTINGS-KIND** `parked` +
  `dependsOnForks (json-projection-format)` — the named consumer of the key-path
  half and retirer of the live `.claude/settings.json` `coverage.unmodeled-surface`
  advisory. Raised two forks: **(json-projection-format)** (settings/MCP/plugin
  need a `json` format harvested + a shared JSON adapter) and **(hook-kind-locus)**
  (hooks live inside settings.json, no file locus). MCP + plugin/marketplace held
  in the json fork, unfiled. PACKAGING-CHANNELS still parked (human creds).
- **Pickable now:** EXTRACTION-VOCAB-GAPS (`open`, in-fence, no shared open file).
- **What's next (human, not a build tick):** curate the agent/command KIND.md +
  PACKAGE.md (un-parks those); harvest the `json` projection format into
  15-kinds.md (un-parks SETTINGS-KIND + the json adapter entry); settle the hook
  locus. The dogfood's SDK/TS-primary migration is still the ledger's ceremony.

Plan continues: no — the queue is reconciled, the inbox drained, and one `open`
entry (EXTRACTION-VOCAB-GAPS) is pickable; hand to build.
