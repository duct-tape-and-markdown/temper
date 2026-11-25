# Does discovery fence Claude Code's worktrees folder?

Driver: a real-harness probe (0.0.25). `coverage.unclaimed-entry` fired on `.claude/worktrees/`.

Verified (code.claude.com/docs/en/worktrees, retrieved 2026-11-25): "By default, the worktree is created under `.claude/worktrees/<name>/` at your repository root". A `WorktreeCreate` hook can move it. The docs also advise "Add `.claude/worktrees/` to your `.gitignore`", and discovery already honors gitignore.

Two answers fit the corpus:
- (a) a fourth fence beside `.git/`, the workspace and a nested governed root. This is runtime state, not authored. But `specs/model/adoption.md`, "Install", grounds discovery's exclusions in ignore rules and governed roots, and names no vendor folder.
- (b) no fence. The one finding stands, and its guidance carries the docs' gitignore advice.

The record's other half needs nothing. The unclaimed scan reads only `.claude/`'s direct children (coverage_note.rs `claude_entries`), so each runtime folder is already one finding, never one per file inside it.
