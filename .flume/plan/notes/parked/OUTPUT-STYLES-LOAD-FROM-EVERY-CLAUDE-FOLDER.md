# OUTPUT-STYLES-LOAD-FROM-EVERY-CLAUDE-FOLDER parked: same-named styles at two depths are refused

Hit the entry's park condition. Built to probe, then reverted:
- locus `.` + `**/.claude/output-styles/*.md`;
- the `named-field-or-file` stem fallback changed to the bare stem (src/extract.rs
  `unit_shape_id`). `fold_file_id` under base `.` keys a nested stem-only style as
  `apps:web:.claude:output-styles:terse`, and a root one as
  `.claude:output-styles:terse`. Not the runtime's name; settings can't select it.

Both tests[] went green. Then the probe: a root `terse.md` plus
`apps/web/.claude/output-styles/terse.md` fails lockless and adopted `check` with
`member.admissibility`: "kind `output-style` declares 2 members that compose to the one
address `output-style:terse`". The runtime loads both, and the nearest wins
(code.claude.com/docs/en/output-styles, retrieved 2026-11-25). So shipping turns a harness
the runtime loads into an error, against 0101. Two `name: X` styles at two depths hit the
same refusal.

Needs a human ruling, for one of these:
(a) a shadowing rule, so one identity can carry more than one placement (nearest-wins, per
    working directory);
(b) a nested style's identity carries its placement (`apps:web:terse`), and the
    `outputStyle` edge resolves a bare name against any depth. This diverges from the
    spec's "identity from `name`, else the file stem";
(c) the root-only locus stays, and nested folders are named as unmodeled.

Two more gaps any choice must answer:
- Emit placement: a `named-field-or-file` name carries no depth, so `splice_name` places
  every authored style at the root `.claude/output-styles/<name>.md`. A nested style that
  check discovers cannot be emitted back where it was found.
- Memory overlap: the `exclude` column is literal-only (kind.rs `excluded_from_row`), and
  `cede_host_units` cedes only to directory-unit hosts. So
  `apps/web/.claude/output-styles/CLAUDE.md` would be claimed by both memory and
  output-style (a `governs-collision` error), with no row-level remedy at any depth.
