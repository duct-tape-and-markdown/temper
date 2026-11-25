<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `supporting-doc` kind

a supporting doc never loads on its own: it reaches context only when its skill's body sends Claude to it, after the skill is invoked — so the body names the file and says when to open it, and a doc the body never mentions is never read. Keep each doc to one concern, so following one pointer loads only what that step needs.

## Locus

Each member is a file nested in a host's unit, at:

- `**/.claude/skills/*/*.md`

## Clauses

- `degree` (advisory): Reference the file from `SKILL.md` — a supporting file the skill's body never points at is invisible: Claude has no way to learn what it holds or when to load it, and it ships as dead weight in the bundle. Any resolved edge from the host skill counts, a mention included; what the edge cannot decide is whether the reference tells Claude *when* to follow it, and that sentence is the point of writing one.
