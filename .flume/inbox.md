<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human, design session): `(kind-harness-axis)` RESOLVED — the
  provider axis (`specs/15-kinds.md` Decision "kind identity carries a
  provider axis"; market evidence in `docs/market-formats.md`). Engine
  consequences, in order:
  (1) PROVIDER-KEY-PARSE — `CustomKind::from_header` accepts a `provider`
  string key (closed shape, any string value; the vocabulary is the market's,
  not the parser's); kind identity becomes the qualified `<provider>.<name>`
  when a provider is declared, bare otherwise; bare references (assembly
  binding, `satisfies` typing, BUILTIN_KINDS lookup) resolve iff exactly one
  kind carries the bare name, with a collision load error naming the qualified
  candidates. Same red-interim shape as FORMAT-KEY-PARSE and
  ACTIVATION-KEY-PARSE: embedded curated files gain `provider` lines only
  AFTER the parser accepts them. NOTE build.rs currently embeds
  `kinds/<name>/KIND.md` (flat); the curated tree moves to
  `kinds/<provider>/<name>/KIND.md` in the human follow-up, so the embed walk
  must accept the nested layout (accepting both during the transition is
  fine).
  (2) HUMAN follow-up (not a build entry): move kinds/skill →
  kinds/claude-code/skill, kinds/rule → kinds/claude-code/rule, add
  `provider = "claude-code"` lines.
  (3) BINDING-QUALIFY — embedded packages bind qualified kind names
  (skill.anthropic → kind claude-code.skill), and the published-package rule
  (a published package binds qualified) becomes a check the bundle/dossier
  path enforces. Depends on (1)+(2).
