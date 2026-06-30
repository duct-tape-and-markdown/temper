# ASSIGNED ENTRY

<entry>
{{ENTRY_JSON}}
</entry>

# THE WHY

Find the section named `{{PER_SECTION}}` (or the nearest equivalent heading) in
the file below. The rest of the spec is context for intent, not scope.

<spec path="{{PER_PATH}}">
!`cat {{PER_PATH}} 2>/dev/null || echo "(spec not found: {{PER_PATH}})"`
</spec>

# CONTEXT

<src-tree>
!`find src -name '*.rs' 2>/dev/null | sort`
</src-tree>

<recent-commits>
!`git log -n 5 --oneline`
</recent-commits>

# TASK

Execute the assigned entry — entry `{{TAG}}`. Implement it completely: no
placeholders, no `todo!()`, no stubbed function bodies.

- Touch only the files declared in `entry.files`. Anything else reverts the commit.
- The acceptance criterion (`entry.acceptance`) must hold.
- Search before assuming "not implemented" (`rg`, `grep`) — the surface may
  already exist under a different module.
- Follow the project's Rust conventions in `.claude/rules/rust.md`: errors via
  `miette`/`thiserror` (no `unwrap`/`expect`/`panic!` on real paths), clippy is
  clean under `-D warnings`, prefer a `clone` over a lifetime fight (this tool is
  I/O-bound — correctness and clarity beat zero-copy).
- Add tests alongside the code. Prefer `insta` snapshots for parse/lint output.
- If the entry needs a dependency not in `Cargo.toml`, add it (Cargo.toml is
  writable) — prefer the crates sanctioned in `SPEC.md §7`.
- If the entry's `per` cite is ambiguous or rests on an unsettled decision, do
  NOT guess: leave it and surface the question (the harness will route it).

# OUTPUT

One commit on this worktree's branch, prefixed `build:`. Imperative subject; the
body explains *why*, not a restatement of the spec.

Gates run automatically after your commit: `cargo fmt --check` (afterCommit),
then `cargo clippy -D warnings` and `cargo test` (afterMerge). A gate failure
reverts your commit and the entry returns to pending — so run
`cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test`
yourself before committing.

Do NOT touch `.flume/plan/pending.json` — the harness updates it post-merge.
