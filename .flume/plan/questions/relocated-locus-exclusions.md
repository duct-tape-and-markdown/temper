# What does a relocated built-in's locus do with the base's excluded paths?

Site: `sdk/src/kind.ts` `relocatedLocus` (~line 733) spreads `...base.locus`, so `relocate(memory, { governs: { root: "docs", glob: "**/CLAUDE.md" } })` emits a row carrying memory's `exclude = [".claude/agents", ".claude/commands", ".claude/rules", ".claude/skills"]`. `src/kind.rs` `excluded_from_row` refuses every one at lock load: `docs/**/CLAUDE.md` neither matches nor reaches beneath `.claude/agents`. So relocating memory off `.` now fails to load. No test relocates memory (`sdk/test/builtins.test.ts` relocates only `rule`, which excludes nothing).

The two sides already disagree: `src/compose.rs` `overlay_builtin_kind` (line 628-630) says a relocated locus carries the row's exclusions, never the built-in's. The corpus doesn't decide between:

1. The relocation drops the base's `exclude`. This matches the engine comment, but a same-reach relocation (`.` + `**/CLAUDE.md`) then collides with `rule` again at `.claude/rules/CLAUDE.md` (the 0084 ownership answer is lost).
2. The relocation keeps the base's excludes that the new template still matches or reaches beneath, and drops the rest. The fact stays true wherever the other loader's folder is still reached.
3. `KindRelocation` gets an authored `exclude` face (the `relocated-locus-exclude` debt), with 1 or 2 as the default.

Recommendation: 2. The exclusion records another loader's ownership (`specs/builtins.md`), and that ownership doesn't change because memory moved. Sections: `specs/model/representation.md` "locus"; `specs/builtins.md` intro paragraph.
