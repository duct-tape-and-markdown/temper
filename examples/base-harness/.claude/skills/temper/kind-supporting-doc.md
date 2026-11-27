<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `supporting-doc` kind

a supporting doc never loads on its own: it reaches context only when its skill's body sends Claude to it, after the skill is invoked — so the body names the file and says when to open it, and a doc the body never mentions is never read. Keep each doc to one concern, so following one pointer loads only what that step needs.

## Locus

Each member is a file nested in a host's unit, at:

- `**/.claude/skills/*/*.md`

## Clauses

The program binds no clauses over it — any default contract the kind ships is what holds it.
