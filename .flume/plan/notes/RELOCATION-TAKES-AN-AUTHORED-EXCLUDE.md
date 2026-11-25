# Relocation takes an authored exclude

Shipped: `KindRelocation.exclude`, appended after the base exclusions the
(possibly moved) template still reaches; an exclude-only delta is a diverging
fact on its own.

The engine needed no logic change. Every `at` row already carries
`governs_root`/`governs_glob`, so an exclude-only row repeats the built-in's
own locus, and `overlay_builtin_kind` writes the row's `exclude` column back
whole. Only the comment in compose.rs changed. The pin
(`builtin_loci::a_relocation_rows_exclusion_leaves_a_fixture_skill_ungoverned`)
covers both an exclude-only row and a moved row.

The SDK refuses an authored exclusion the locus cannot subtract: not starless,
off the template, or already excluded. That check mirrors `excluded_from_row`
and runs at emit time, so an author sees the error there and not when the
engine loads the lock.

Dogfood caveat: the shipped skill locus is still `.claude/skills` + `*/SKILL.md`,
which never reaches `tests/fixtures/extract_equivalence`. An exclude-only
relocation naming that fixture is refused today. Until the nested skill locus
(`**/.claude/skills/*/SKILL.md`, per builtins.md) ships, the dogfood has to
pair the exclude with that `governs`. The SDK test pins that combined form.
