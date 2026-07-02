+++
[satisfies.collaboration-discipline]
rationale = "the unconditional rule that makes pushback the default: challenge gaps, never paper over them — the discipline every session and phase agent inherits"

[provenance]
source_path = "./.claude/rules/collaboration.md"
import_hash = "79f88188eed7558301cf4c974f0748a20687b315df60bb19440faca7d73a8c43"
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
