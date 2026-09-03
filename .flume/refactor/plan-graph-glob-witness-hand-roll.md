## Surface

`src/glob.rs`'s own doc comment claims exclusivity: `compile_glob` is "the
one glob-matching surface every caller shares... graph's `paths-match`
liveness test" is named as a consumer (glob.rs:1-14) — and CLAUDE.md's tech
stack pins `globset` as "the one glob engine — already inside `ignore`;
never hand-roll matching." `src/graph.rs`'s `mention-reachable` containment
check (`uncontained`, graph.rs:469; `is_scope_contained_in_gate`,
graph.rs:482) instead carries its own ~250-line hand-rolled glob-syntax
parser, independent of `globset`, added this window by two properly-ruled
entries (MENTION-REACHABLE-SUBSET-CONTAINMENT, GH #33, and
GRAPH-SCOPE-WITNESS-GLOB-VOCABULARY — both plan-filed, human-ruled, shipped
clean per their own commit bodies; not drift, the entries just didn't name
the duplication as part of their ask):

- `representative_paths_for_glob` (graph.rs:514) — segments a glob on `/`,
  synthesizes `**`-depth witnesses.
- `process_glob_segment` (graph.rs:588) / `expand_braces` (graph.rs:603) —
  hand-parses `{a,b,c}` brace-alternation syntax that `globset`'s own
  `Glob` parser already understands when compiling (it just doesn't expose
  the parsed alternatives publicly).
- `concretize_segment` (graph.rs:629) / `concretize_char_class`
  (graph.rs:671) / `is_in_char_class` (graph.rs:724) — hand-parses `[...]`
  character-class syntax (ranges, negation) to synthesize a representative
  char, again independent of `globset`'s own class parser.

`is_scope_contained_in_gate` does call `crate::glob::compile_glob` for the
*matching* half (testing a witness path against the gate globs), so the two
engines are stitched together in one function: parse with the hand-roll,
match with `globset`.

The underlying mechanic — "is scope glob's match-set a subset of gate glob
set's match-set" — is genuinely not something `globset` answers directly
(it matches, it doesn't compare languages), so this isn't a case of a
sanctioned crate carrying the exact mechanic untouched. But the *syntax
parsing* half (braces, character classes, wildcard segments) that feeds the
witness synthesis duplicates parsing knowledge `globset`'s own compiler
already embeds — that's the concrete "Libraries before hand-rolls" tension
(`specs/process/engineering.md`), not the subset-containment idea itself.

## Observed at

e8c160c3

## Suggested consolidation

A human/design-session pick, one idea is enough to start from: justify the
duplication explicitly as the engineering.md "pinned semantics" exception
(witness-sampling containment is a mechanic outside `globset`'s scope, so
hand-parsing glob syntax a second time to drive it is defensible) and say
so in graph.rs's doc comments, so a future sweep doesn't re-flag the same
~250 lines — or find whether `globset`/a sibling crate in the sanctioned
set (transitively, via `ignore`) exposes enough of its parsed glob AST to
drive witness synthesis off the same parse `compile_glob` already performs,
collapsing the second parser. Either way the fix is a doc-comment or a
consolidation, not a behavior change — the tested subset-containment
semantics (tests/graph.rs:804-850) stay as ruled.
