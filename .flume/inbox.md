<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): EMBED-NESTED-WALK — the curated-move half of the provider
  axis is blocked on build.rs: `embed()` walks exactly `<tree>/<name>/<marker>`
  and keys by dir name, so `kinds/claude-code/skill/KIND.md` would be silently
  skipped. File a slice: the kinds walk accepts BOTH the flat `<name>/KIND.md`
  and the nested `<provider>/<name>/KIND.md` layout (transitional tolerance);
  a nested embed keys as the qualified `"<provider>.<name>"`; flat keys stay
  bare. Consumers of BUILTIN_KINDS must reach qualified entries through the
  bare-name resolution PROVIDER-KEY-PARSE shipped (c52df4f) — `skill` resolves
  to `claude-code.skill` when unique. Packages tree stays flat (package names
  are their own namespace). After it drains, the human moves kinds/skill →
  kinds/claude-code/skill (+ rule) and adds `provider = "claude-code"` lines;
  then BINDING-QUALIFY un-parks.
