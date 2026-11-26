<!-- temper: managed projection — a direct edit here is drift; edit the owning .temper/ module or document and re-run temper emit, never this generated file. -->

# The `rule` kind

a rule reaches context on one of two schedules: with no `paths` frontmatter it loads at launch, beside `CLAUDE.md`; with `paths` it loads the first time Read, Write or Edit touches a matching file, once per session. So keep a rule to facts Claude should hold whenever the rule is loaded — concrete enough to verify ('use 2-space indentation', not 'format code properly'). If an entry is a multi-step procedure or only matters occasionally, it belongs in a skill, whose body loads only when invoked, rather than a rule, which loads whole once its scope opens. Prefer path-scoped rules when one convention governs scattered paths; prefer per-directory CLAUDE.md when directory owners maintain their own. Treat rules like code: prune them when behavior drifts, and test a change by watching whether Claude's behavior actually shifts.

## Locus

Each member is its own document, at:

- `.claude/rules/**/*.md`

## Clauses

The program binds no clauses over it — any default contract the kind ships is what holds it.

## Domains

- `conduct` (a member of this kind fills it) — read [domain-conduct.md](domain-conduct.md).
- `standards` (a member of this kind fills it) — read [domain-standards.md](domain-standards.md).
