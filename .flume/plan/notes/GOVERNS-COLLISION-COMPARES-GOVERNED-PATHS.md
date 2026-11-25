# Governs collision judged per discovered document

Shipped `admissibility::document_governs_collision_diagnostics`, wired in
`gate.rs` beside the kept root+glob string check. It walks each file kind's
`discover_kind_files` (Honored flavor) off the run's shared `Discovery`, so it
adds no tree walk; `import.rs` needed no new surface.

Shape choices plan may want to revisit:
- One finding per *document* (subject = the harness-relative path), naming
  every kind that reaches it. A pair of kinds colliding at N documents draws
  N findings; a per-pair roll-up was not chosen because the remedy is a
  per-path exclude.
- A document reached only by kinds sharing one identical root+glob is left
  to the string check (no double report). A document reached by an
  identical-locus pair plus a third distinct kind is reported here too.
- `skill`'s `bare_root_file` counts as part of its reach, so a custom
  `.`+`*.md` kind collides with a real root `SKILL.md`.

This repo's own `temper check` draws no governs collision.
