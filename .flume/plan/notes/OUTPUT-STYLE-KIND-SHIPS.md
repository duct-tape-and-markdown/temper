# output-style shipped; nested style dirs and a new unit shape

- Identity "name, else stem" had no unit shape: `named-field` faults when
  the field is absent. Added `named-field-or-file(<field>)` (SDK
  `UnitShape`, lock label, `kind::UnitShape::NamedFieldOrFile`); the one
  id derivation in `extract::unit_shape_id` falls back to the file fold.
- Locus is `.claude/output-styles/*.md`. The docs (output-styles,
  retrieved 2026-11-25) are silent on subfolders inside `output-styles/`,
  but say project styles "load from every `.claude/output-styles/` between
  the working directory and the repository root", nearest one winning on
  a name clash. That any-depth placement is unmodeled: it needs the
  skill-style `**/` locus plus a shadowing rule for duplicate names. Plan
  should decide whether that is an entry.
- `memory`'s exclude gains `.claude/output-styles` (decision 0084's
  loader-owned-folder rule), so a `CLAUDE.md` style is never memory too.
  The dogfood `.temper/lock.toml` will move on that row and the new kind
  row at the next human re-emit; `examples/base-harness` was re-emitted.
- Built-in styles (`Proactive`, `Concise`, `Explanatory`, `Learning`,
  case-sensitive) per the same page: input for
  OUTPUT-STYLE-IS-SELECTED-BY-SETTINGS's dangling-edge exemption.
- The lock header miscounted the empty contracts (`command` was
  missing); corrected to five.
