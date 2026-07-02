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
- BUG (found dogfooding): `temper session-start .` fails on a project with a
  registered custom kind — the one-shot import lands in a temp workspace that
  never carries `.temper/kinds/<name>/KIND.md` (or `.temper/packages/`), so the
  registration dangles (kind::missing_definition). The one-shot path must read
  the project's authored definitions/packages even though members import to
  temp. Repro: `temper session-start .` at temper's own root. High priority —
  the SessionStart hook is now installed here and errors loudly every session
  until fixed.
