+++
[satisfies.collaboration-discipline]
rationale = "the unconditional rule that makes pushback the default: challenge gaps, never paper over them — the discipline every session and phase agent inherits"

[provenance]
source_path = "./.claude/rules/collaboration.md"
import_hash = "b595e34a60c47d903564837e88347dda9132808e863af255b9ca9b8840d37fed"
+++
# Collaboration — pushback is the point

`temper` is a design-led project. The harness enforces *mechanics*; judgment is
yours. The most useful thing you do is surface what's wrong or undecided — not
fill it silently.

## Challenge gaps, never paper over them

- If a spec section is ambiguous, under-specified, or rests on an unsettled
  decision, **stop and surface it** — do not invent intent to keep moving. In
  `build`, leave the entry and raise an open question; in conversation, say so
  plainly and ask.
- A derived layer never invents intent absent from its source. Plan does not
  invent requirements the spec doesn't carry; build does not invent behavior the
  entry doesn't name.
- Open questions are keyed `(slug)` in `.flume/plan/open-questions.md`. An entry
  resting on one declares `dependsOnForks: ["slug"]` and waits — building onto an
  undecided foundation is worse than not building.

## External facts are cited, never guessed

- A claim about a real-world format or behavior — Claude Code's file layout, a
  frontmatter schema, a registry's rules, an API's shape — is an **external
  fact**, not background knowledge. It carries its source (doc URL, retrieved
  date) at the point of claim: in the spec section, the pending entry, or the
  comment that encodes it.
- If you cannot cite it, **verify before encoding it** (fetch the current
  docs), or mark it `UNVERIFIED` and surface the gap. An uncited guess wearing
  spec authority is how the import-locus bug shipped: the spec said
  `skills/*/SKILL.md`, reality says `.claude/skills/`, and nothing forced
  anyone to look.
- The product already holds this bar for its own std-lib (per-clause `source`,
  `specs/10-contracts.md`). The project holds itself to the bar it ships.

## Investigation discipline

- When asked to investigate, **investigate and report — modify nothing.**
- Read the disk artifact to answer "did X ship / is gate Y green", never the git
  log. Git log is orientation, not authority.
- Search before claiming "not implemented" (`rg`/`grep`) — the surface may exist
  under another module.

## Scope honesty

- Don't quietly expand scope past the assigned entry to "improve" adjacent code.
- If you cut a corner (a `// TODO`, a deferred case, a weaker check), say so
  out loud in the commit body or the response — a silent gap reads as done.
