<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `hook` kind

keep a handler's `type` among `command`/`http`/`mcp_tool`/`prompt`/`agent`; a `command` handler needs a `command`, an `http` handler a `url`; the `matcher` filters tool-scoped events and is inert on events that carry no tool (`UserPromptSubmit`, `Stop`, and their siblings).

## Locus

Each member is its own document, at:

- `.claude/settings.json`

## Clauses

- `enum` on `event` (required): A hook keys under its lifecycle event; an event outside the documented set is dead configuration — Claude Code silently never fires a hook under an unrecognized event. If this is a newly-documented event, re-fetch code.claude.com/docs/en/hooks and extend temper's cited set rather than working around the finding.
