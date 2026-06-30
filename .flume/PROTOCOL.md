# Flume Protocol — author project conventions

Runtime mechanics (baton, gates, handoff, pending schema) live in `chain.ts` and
the `@dtmd/flume` runtime. This file holds the project-side conventions the chain
config doesn't encode.

## The chain

`specs/` (evergreen) → `.flume/plan/` → `src/` (+ `tests/`, `Cargo.toml`, docs) → git log

The `specs/` corpus is the evergreen source of truth (`specs/90-spec-system.md`),
authored only in interactive sessions under explicit direction, never by an
autonomous phase. It is **not** a release line: plan reconciles code against the
living corpus every tick — there is no frozen ship target. `specs/00-intent.md`
is the north star.

| Layer | Author | Phase | Commit prefix  |
| ----- | ------ | ----- | -------------- |
| spec  | human  | —     | (any)          |
| plan  | plan   | plan  | `plan:`        |
| code  | build  | build | `build:`       |

Harness-authored commits (post-merge ship) use `chore(flume):`.

## Two harnesses live here — don't confuse them

This repo carries **two** harnesses with different owners:

1. **The flume harness** (`.flume/`) — the build pipeline. Authored by humans;
   never edited by the `build` phase.
2. **The Claude Code harness** (`.claude/`, `CLAUDE.md`) — the *product domain*:
   the very artifacts `author` is built to project. It is also `author`'s first
   dogfood fixture and the environment the build agents themselves run inside, so
   it is hand-curated to an exemplary standard. The `build` phase never edits it;
   changes flow through human `chore(harness):` commits.

`writablePaths` in `chain.ts` enforces both exclusions mechanically.

## Rust gate model

Gate placement follows CHAIN-AUTHORING §2, adapted for cargo (compilation is the
expensive step):

- **afterCommit:** `cargo fmt --all --check` only — no compile, safe to run
  N-wide under fanout.
- **afterMerge:** `cargo clippy --all-targets -- -D warnings` and `cargo test` —
  serial on the trunk, no N-wide contention; a failure reverts only the offending
  entry.

No `setupWorktree`: cargo shares its registry cache via `~/.cargo`; only `target/`
is per-worktree (the cold compile kept off the parallel path on purpose).

## Plan continuation marker

Plan processes the *delta* since the last `plan:` commit. When the delta exceeds
one good tick, `state.md` ends with `Plan continues: yes — <reason>` and the
harness re-wakes plan; `Plan continues: no` (or absence) hands to build or
hibernates. The regex `^Plan continues:\s*yes\b` (in `chain.ts`) is load-bearing.

## Disk vs git log

To answer "did X ship?" or "is gate Y green?", read the disk artifact
(`pending.json`, the `src/` file, `cargo` output). Never grep commit messages.
Git log is orientation, not authority.

## Non-negotiables

- Build commits per entry to the trunk after green validation.
- NEVER force-push, amend pushed commits, or `--no-verify`.
- NEVER modify files when asked to investigate — investigate and report.
- Search the codebase before implementing — don't assume not implemented.
- Never silently fill a spec gap — surface it as an open question.
