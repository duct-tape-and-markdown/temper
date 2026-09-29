## Surface

`coverage.unmodeled-surface` is retired (this tick), but three sites outside
UNMODELED-SURFACE-ADVISORY-RETIRES's fence still name it and now assert against
a finding class no code can emit — green, but vacuous:

- `tests/it/mcp_server_kind.rs:167` — `findings_for(.., "coverage.unmodeled-surface")`
  filtered for `.mcp.json`; the surviving half of the case is the `mcp (1)` /
  `mcp-server (2)` disclosure beside it.
- `tests/it/mcp_server_kind.rs:201` — same, asserting the whole class empty; the
  case's real subject is the `root.locus-declared` finding below it.
- `tests/it/mcp_kind.rs:284` — same, with the comment claiming a "gap left to name".

And one stale comment: `src/gate.rs:733` still describes the coverage note as
naming "known Claude Code surfaces present on disk that no kind governs", which
is no longer what the call below it does.

## Observed at

cc46bbb1 (HEAD when observed) — the entry's census listed the whole `rg unmodeled`
set but its `files[]` carried neither `mcp_*_kind.rs` nor `gate.rs`, so they were
out of fence.

## Suggested consolidation

Delete the three assertions (each case keeps a real subject without them) and
re-point `gate.rs`'s comment at the two surviving classes — the disclosure and the
unclaimed `.claude/` entry.
