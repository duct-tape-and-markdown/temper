# Plan state

- **Phase:** reconcile. HEAD e6be469.
- **Last shipped:** DIRECTIVE-BACKING-BASE-DIR (build 590b3e0 / chore e6be469) —
  the two-step `check` path now derives a real `base_dir` (bare
  `Path::new("temper.toml").parent()` was `Some("")`, walking an empty world and
  forging false `graph.directive-unbacked` findings on every real `@import`).
  Dropped from the queue.
- **This tick:** **dropped COMMUNITY-DOCS — it has shipped.** Both launch docs
  exist tracked in `.github/` (`CONTRIBUTING.md`, `SECURITY.md`), satisfy
  55-offering's two Decisions (two-sided AI-authorship disclosure;
  demonstrate-not-speculate evidence bar + private reporting), and `.github/**`
  is already inside build's fence (chain.ts:156) — so the entry's premise (files
  missing, fence must be widened) was false. Prior ticks checked root only and
  missed the `.github/` versions. Re-verified the 4 carried parked/deferred
  entries on disk — all accurate (no `ignore` crate; `Field` flat / no `Fenced`;
  BUILTIN_KINDS=`["skill","rule"]`; package.json still `temper-flume-harness`/
  private, only `.github/workflows/temper.yml` CI, no release.yml).
- **Pickable now:** none. All 4 queue entries are human-gated (WALK-IGNORE
  parked on a spec Decision + `ignore` sanction; PACKAGING parked on release
  creds; EXTRACTION-VOCAB-GAPS + AGENT-KIND deferred, no consumer). v0.1 launch
  progress now needs human action, not a build tick.
- **Accepted debt (not filed as build work):** (1) 55-offering says the launch
  docs are "root-level"; disk has them in `.github/` (README wired there;
  GitHub recognizes both) — a spec-wording reconciliation for the human, not a
  file move (moving would break the README links). (2) Enabling GitHub private
  vulnerability reporting is a repo setting (human), not a file.
- **Operational note (accepted):** the session-start 19 `requirement.dangling`
  findings are a **stale global binary** (`~/.cargo/bin/temper`, older than
  `target/debug/temper`); a fresh `cargo build && ./target/debug/temper check
  .temper` shows 0. `cargo install --path .` clears it.

Plan continues: no — inbox empty, COMMUNITY-DOCS reconciled out as shipped, the
remaining queue is entirely human-gated. No pickable entry exists and
re-planning won't create one; hand back for human unblocking.
