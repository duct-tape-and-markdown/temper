<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->
- The std-lib packages are authored: `packages/{skill.anthropic,rule.anthropic}/PACKAGE.md`
  (966147d — product source per specs/10-contracts.md, clause-sourced, guidance
  colocated). EMBED-BUILTIN-PACKAGES' parked reason is satisfied: un-park it. Its
  shape per spec: embed the authored PACKAGE.md sources (include_dir/build.rs is
  the sanctioned addition), bind by name, retire contracts/*.toml and the bare
  `rule` package name (specs/20-surface.md binds rule -> rule.anthropic).
- `(launch-front-door-docs)` is RESOLVED — AGENTS-MD (contributor doc, no
  .claude/CLAUDE.md edits) and CHANGELOG-STUB are fileable root-doc entries.
