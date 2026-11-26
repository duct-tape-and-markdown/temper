# Worktrees finding quotes the gitignore advice; the scan now reads the root .gitignore

The pin "a gitignored claude worktrees folder fires no unclaimed entry" did not
hold on the base for the docs' own spelling: the unclaimed-entry scan walked
rooted at `.claude/` with `parents(false)`, so a harness-root `.gitignore`
naming `.claude/worktrees/` (and the repo's `info/exclude`, whose `.git` was
looked up under `.claude/`) was never read. Only a `.claude/.gitignore` worked.
A remedy that does not silence its finding breaks "a finding is true for its
reader" (contract.md, Read verbs), so the fix landed with the entry:
`src/coverage_note.rs` `claude_walk` roots both `.claude` walks (stray scan and
the fully-reached-dirs claim) at the harness root, pruned to `.claude`.

Side effect worth knowing: a root `.gitignore` that ignores `.claude/` whole
now silences the whole stray scan (before, the root's file was never consulted).

The remedy is `cited_remedy` in the same file — a single-row match today; a
second documented per-entry remedy should turn it into a cited table.
