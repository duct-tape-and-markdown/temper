# outputStyle edge shipped; a new `provided` column, and two gaps it exposed

Shipped: `settings` and `settings-local` declare `outputStyle` -> `output-style`.
Matching is exact on the authored bytes. Built-in styles (`default`, `Proactive`,
`Concise`, `Explanatory`, `Learning`, verified on code.claude.com/docs/en/output-styles
2026-11-25; no longer UNVERIFIED) ride a new optional edge fact, `provided`. It is
on SDK `EdgeField`, the lock's `[[declaration.assembly]]` edge row and
`compose::Edge`. A missing target the edge lists as provided neither resolves nor
dangles (`graph::resolved_edges`).

For plan / spec:
- `provided` is a new edge primitive that `specs/model/contract.md` "edge" does not
  name. The entry asked for the exemption, and this is the smallest home for it. The
  spec should record it, or pick another.
- A lockless `check --harness` resolves no built-in edge. `gate.rs` reads assembly
  edges only off the committed lock, never the embedded built-in lock. So the stranger
  gate stays silent on a dangling `outputStyle`, and on a dangling
  `installed-plugin` marketplace too. This was already true before this change. Tests
  stand the harness up with a committed copy of the built-in declarations.
- Styles at user scope (`~/.claude/output-styles`) and plugin styles are loaded by
  the runtime, but they read as dangling (error). This follows the
  installed-plugin -> known-marketplace precedent. A `settings.local.json` naming a
  personal style is the likeliest false positive. The spec says nothing on
  out-of-corpus targets.
- The edge has no intent declared, so the bundled reference renders "no intent
  declared" for it.
