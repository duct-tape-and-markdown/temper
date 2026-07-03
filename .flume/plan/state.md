# Plan state

- **Phase:** reconcile. HEAD 87a34f8.
- **Last shipped:** the memory kinds (chore 86d5b70) — the four curated files
  (`kinds/{claude-code,agents-md}/memory/KIND.md`, `packages/memory.{anthropic,agents-md}/PACKAGE.md`)
  landed on disk; agents-md joins claude-code as the first foreign provider. Verified on disk.
- **This tick:** drained the inbox line (CHECK-MEMBERS-ALL-KINDS) after reproducing it:
  a 251-line CLAUDE.md via `check --harness` fires **zero** clause advisories, yet
  `import` projects it to `CLAUDE/MEMORY.md` — discovery works, but `check_gate`
  hardcodes the skill/rule pair so no memory member is dispatched to `memory.anthropic`.
  Filed **CHECK-MEMBERS-ALL-KINDS** (open, pickable) — generic per-kind validation loop +
  qualified-identity memory floor bindings. Corrected the stale premise that MEMORY-KIND
  parks on a human file commit: the four files are on disk, so MEMORY-KIND now **blockedBy
  CHECK-MEMBERS-ALL-KINDS** (flip-ceremony validation). Carved the read-family half of the
  hardcoding into a new fork `(builtin-workspace-qualified-key)` — no live consumer, real
  keying question. Refreshed the bootstrap-fence datum.
- **Operational note (accepted, not queued):** the 17 `requirement.dangling`
  session-start findings are a **stale installed binary** — `cargo install --path .`
  clears them; a freshly-built `temper check .temper` is clean.
- **Pickable now:** CHECK-MEMBERS-ALL-KINDS (the one open engine entry). Blocked:
  MEMORY-KIND (blockedBy CHECK-MEMBERS-ALL-KINDS). Parked (human action): PACKAGING-CHANNELS
  (release creds), COMMUNITY-DOCS (fence-widen + private reporting). Deferred (no consumer):
  EXTRACTION-VOCAB-GAPS, AGENT-KIND.

Plan continues: no — inbox drained, queue reconciled, one open entry filed
(CHECK-MEMBERS-ALL-KINDS). Hand to build; MEMORY-KIND unblocks once it ships.
