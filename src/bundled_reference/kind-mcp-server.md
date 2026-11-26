<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `mcp-server` kind

The kind declares no counsel.

## Locus

Each member is its own document, at:

- `.mcp.json`

## Clauses

- `enum` on `type` (required): A server's `type` names its transport; a value outside the documented set is one Claude Code cannot honor. Absent reads as `stdio`, the documented default. If this is a newly-documented transport, re-fetch code.claude.com/docs/en/mcp and extend temper's cited set rather than working around the finding.
- `when` (required): Where a server's transport is `stdio` — the default an entry with no `type` reads as — the body below holds the field a local process requires: the command that starts it.
- `when` (required): Where a server's transport is remote (`http`, `streamable-http`, `sse` or `ws`), the body below holds the field every remote transport requires: the endpoint URL.
