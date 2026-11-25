# What judges "two kinds never govern one path" now that it is a path claim, not a glob claim?

Driver: retired-claim delta of f5c391d3 (0081). `representation.md`, "locus", used to say two kinds never *share a `governs` glob*. It now says two kinds never *govern one path*. The code still enforces the old claim: `admissibility::governs_collision_diagnostics` (src/admissibility.rs:701) buckets kinds by the exact `(governs.root, governs.glob)` string pair. Two kinds whose templates overlap without being spelled alike pass:

- A custom kind at root `.claude`, glob `rules/*.md` and the built-in `rule` (`.claude/rules`, `*.md`) govern the same paths. No finding.
- The built-ins overlap among themselves: `memory` (`.`, `**/CLAUDE.md`) matches `.claude/rules/CLAUDE.md`, which `rule` (`.claude/rules`, `*.md`) also governs. The same goes for `agent` (`**/*.md`) and `command`. `excluded` cannot carve this out, because it takes starless literal paths only (`CustomKind::excluded`, src/kind.rs:135).

The fork:

- **(a)** Judge overlap at load, over templates. 0081 made templates decidable, so the judgment can be static. Under it the built-in `memory` locus refuses unless it gains a template-level exclusion, which is new syntax 0081 declined to add.
- **(b)** Judge at the path. A discovered document that two kinds' loci both match is a finding at check, naming both kinds. Load stays string-keyed.
- **(c)** Keep the string check and amend the spec back to a glob claim. Then the `memory`/`rule` overlap stays unjudged.

Stance: (b). It is the claim the spec now states, judged where it becomes decidable without new pattern syntax. The objection: (b) is loud only once a colliding file exists, and the built-in overlap means a harness with `.claude/rules/CLAUDE.md` would start failing. That makes it a ruling on which kind owns that path, which the corpus does not state.
