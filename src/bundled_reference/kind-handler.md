<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `handler` kind

The kind declares no counsel.

## Locus

Each member is embedded in a host's document, at:

- `.claude/settings.json`

## Edges

### Out

- `runs` → `tool`: no intent declared.

## Clauses

- `required` on `type` (required): A handler names how it fires, and the documented common-fields table gives `type` no default — so a handler carrying none is dead configuration: nothing to run, and no per-kind requirement for temper to hold it to either. Name one of the documented five (`command`, `http`, `mcp_tool`, `prompt`, `agent`).
- `enum` on `type` (required): A handler's `type` names how the hook fires; a value outside the documented five (`command`, `http`, `mcp_tool`, `prompt`, `agent`) is a handler Claude Code cannot run. If this is a newly-documented handler kind, re-fetch code.claude.com/docs/en/hooks and extend temper's cited set rather than working around the finding.
- `when` (required): Where a handler's `type` is `command`, the body below holds the field that type requires: the command to run.
- `when` (required): Where a handler's `type` is `http`, the body below holds the field that type requires: the endpoint the event's JSON input is POSTed to.
- `when` (required): Where a handler's `type` is `mcp_tool`, the body below holds the fields that type requires: the connected server and the tool to call on it.
- `when` (required): Where a handler's `type` is `prompt` or `agent`, the body below holds the field both types require: the prompt text sent to the model.
