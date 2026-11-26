# What does an unrepresented harness's grouped script note cover?

Driver: 0104 ("an unrepresented harness reports such scripts as one grouped coverage note (0103)"). The represented half is A-SETTINGS-SCRIPT-IS-A-FILE-TOOL.

An unrepresented harness declares no tools, and finding a script by parsing a hook command or `statusLine` for a path is mining (intent invariant 1). So temper cannot tell a script folder from any other stray under `.claude/`. Two readings fit the corpus:

- (a) every `coverage.unclaimed-entry` in an unrepresented harness collapses into one note, "represent the harness and declare each script a `{ file }` tool". 0103 supports this: the per-entry remedy cannot work there. But it also swallows strays such as `.claude/worktrees/`, whose gitignore remedy does work there (WORKTREES-FINDING-QUOTES-THE-GITIGNORE-ADVICE).
- (b) the per-entry finding stays, and the grouped note is added only when settings carry a hook or status line command at all. That is decidable without mining, but the note cannot count the scripts.

Checked: 0103, 0104, `contract.md` "Read verbs", `coverage_note.rs` (the unclaimed scan reads `.claude/`'s direct children only). Recommendation: (b). It keeps a remedy that works and claims nothing it cannot count. A ruling amends `builtins.md`, "Tool".
