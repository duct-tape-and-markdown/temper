+++
[provenance]
source_path = "././AGENTS.md"
source_hash = "2b3d873ddccae994d288fd155bfe98c3a96bdfcaa20e84366c920afbd66b17fa"
+++
# AGENTS.md

Contributor-facing agent instructions for `temper` — a Rust CLI that
type-checks a Claude Code harness (`.claude/` — skills, rules, hooks,
settings) against declared contracts. If you are a coding agent working a
contribution, this file is for you. (This repo's own `CLAUDE.md` is not a
duplicate of this file: it is the operating harness for the project's internal
agent pipeline, and it is hand-curated — do not edit it, or anything under
`.claude/`, in a contribution.)

## Commands

- `cargo build` — compile.
- `cargo test` — the test suite; prefer `insta` snapshots for parse/diagnostic
  output (`cargo insta review` to accept changes deliberately).
- `cargo clippy --all-targets -- -D warnings` — the lint bar; warnings are
  errors here.
- `cargo fmt --all` — format (`--check` is what CI runs).
- `cargo run --quiet -- check` — temper checking its own harness surface; must
  stay green (advisories are fine, `required` violations are not).

## Architecture, briefly

Logic lives in the library crate; `src/main.rs` is a thin `clap` dispatch.
One artifact kind per module (`skill.rs`, `rule.rs`). The load-bearing halves:
`contract.rs`/`engine.rs` (the closed predicate vocabulary and its evaluation),
`kind.rs`/`extract.rs` (the extraction algebra), `document.rs` (the
`+++`-fenced member document), `import.rs`/`drift.rs` (the surface and its
three-state drift engine), `compose.rs`/`roster.rs`/`coverage.rs`/`graph.rs`
(the assembly: bindings, requirements, satisfier sets, the relation graph).
The evergreen intent lives in `specs/` (start at `specs/intent/00-intent.md`) — specs
are human-authored; do not edit them in a PR.

## Rules

- ALWAYS run fmt, clippy, and the tests before proposing a change; a red gate
  is an unfinished change.
- NEVER use `unwrap`/`expect`/`panic!` on real code paths — model errors with
  `thiserror`, surface them with `miette`.
- NEVER re-render markdown bodies or companion files — they are copied
  byte-for-byte; only structured headers are rewritten, via `toml_edit`.
- Comments cite the relevant `specs/` section and state only what the code
  adds; never restate spec narrative.
- Search before claiming something is not implemented — the surface may exist
  under another module.
- If the spec section you are working against is ambiguous, stop and say so in
  the PR rather than inventing intent.

## Contributions

Read `.github/CONTRIBUTING.md` before opening anything — it carries the
project's two-sided AI policy: this codebase is largely agent-authored under
human direction and gated commits, and inbound AI-assisted work is welcome
**with disclosure**, provided you understand and can defend the change without
the assistant. Security reports go through `.github/SECURITY.md` and must
demonstrate, not speculate.
