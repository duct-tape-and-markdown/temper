<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `supporting-doc` kind

a supporting doc never loads on its own: it reaches context only when its skill's body sends Claude to it, after the skill is invoked — so the body names the file and says when to open it, and a doc the body never mentions is never read. Keep each doc to one concern, so following one pointer loads only what that step needs.

## Locus

Each member is a file nested in a host's unit, at:

- `**/.claude/skills/*/*.md`

## Clauses

- `degree` (advisory): No typed reference reaches this file: no resolved edge from the host skill points at it. A supporting file the skill's body never points at is invisible: Claude has no way to learn what it holds or when to load it, and it ships as dead weight in the bundle. Reference it from `SKILL.md` with `mentionOf(doc)`, or its path mention (`mentionOf`'s `path` form) where the body links the file; a hand-typed link in prose is no edge. What the edge cannot decide is whether the reference tells Claude *when* to follow it, and that sentence is the point of writing one.
