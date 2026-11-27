# `root.locus-declared` help names only one of its two answers

Observed on the dogfood once RULE-CLAIMS-NESTED-CLAUDE-RULES shipped. The nested rule claim reached rule files held as test data in `tests/fixtures/extract_equivalence/.claude/rules/` and `examples/demo/harness/.claude/rules/`. It drew three `root.locus-declared` findings and one `rule.forbidden-keys` finding, from the demo's deliberate Cursor-style `globs` key.

The help says "declare the member in the program and re-emit". For a document held as data, the answer `builtins.md` gives is the other one: "An author may add exclusions to a built-in through its relocation. That is how … a fixture tree holding harness files is left ungoverned." The dogfood took that path, with `relocate(rule, { exclude: [...] })` in `.temper/kinds.ts`. An adopter with fixtures meets the same findings on any upgrade that widens a built-in's claim, and the help points them at the wrong fix.

The spec already states both answers, so this is a code gap. The finding's help should name both: declare the member, or exclude its tree through the kind's relocation.
