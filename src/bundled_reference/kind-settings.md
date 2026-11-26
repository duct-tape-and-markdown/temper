<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `settings` kind

The kind declares no counsel.

## Locus

Each member is its own document, at:

- `.claude/settings.json`

## Edges

### Out

- `runs` → `tool`: no intent declared.

## Clauses

- `type` on `permissions` (required): `permissions` is the tool-permission object — `allow`, `ask` and `deny` rule arrays plus `defaultMode`. A value that is not an object carries no rules Claude Code can read, so the rules a whole team relies on silently apply nothing.
- `type` on `env` (required): `env` is a map of environment variables — string keys to string values — applied to every session and its subprocesses. A non-object value is not a shorter spelling of one variable; it is a shape the loader cannot expand, so none of the variables take effect.
- `type` on `attribution` (required): `attribution` is the commit/PR attribution object — `commit`, `pr` and `sessionUrl`. A non-object value configures none of them, so Claude Code falls back to its default trailer with no error, which is exactly the outcome a project setting this key was avoiding.
