<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `agent` kind

The kind declares no counsel.

## Locus

Each member is its own document, at:

- `.claude/agents/**/*.md`

## Clauses

- `required` on `name` (required): Every subagent declares a `name` — its unique identifier. Claude Code binds identity to this field alone, never the filename, so a nameless subagent cannot be delegated to.
- `allowed_chars` on `name` (required): Lowercase letters and hyphens only — no digits, unlike a skill's `[a-z0-9-]` name. Hooks receive this value as `agent_type`.
- `unique-name` (required): Keep `name` values unique across the whole tree — when two files in one scope declare the same name, Claude Code loads only one of them, silently shadowing the other.
- `required` on `description` (required): The description is how Claude decides when to delegate to this subagent — write it so the trigger is unambiguous.
