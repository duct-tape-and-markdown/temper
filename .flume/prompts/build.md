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
!`{ find src tests -name '*.rs'; find sdk/src sdk/test -name '*.ts'; } 2>/dev/null | sort`
</src-tree>

<recent-commits>
!`git log -n 5 --oneline`
</recent-commits>

# TASK

Execute the assigned entry — entry `{{TAG}}`. Implement it completely: no
placeholders, no `todo!()`, no stubbed function bodies.

- `entry.files` is the *planned* scope, not a cage. The real boundary is the
  writable paths in the `<harness>` block — staying inside those never reverts.
  **If reaching green needs a file `entry.files` didn't list — almost always an
  existing test your change breaks — edit it.** An under-scoped `entry.files` is a
  planning miss; shipping red, or bailing with no commit, is worse. Only writes
  *outside the harness writable paths* revert.
- The acceptance criterion (`entry.acceptance`) must hold.
- Search before assuming "not implemented" (`rg`, `grep`) — the surface may
  already exist under a different module.
- The entry's premise describes the tree it was scoped against, and the
  queue keeps moving: when `notes` carries `scoped at <sha>`, run
  `git log <sha>..HEAD -- <the entry's files>` before anything else — an
  already-landed fix narrows the entry to its remainder, or empties it
  (leave it uncommitted and say so in the report).
- Follow the project's Rust conventions in `.claude/rules/rust.md`: errors via
  `miette`/`thiserror` (no `unwrap`/`expect`/`panic!` on real paths), clippy is
  clean under `-D warnings`, prefer a `clone` over a lifetime fight (this tool is
  I/O-bound — correctness and clarity beat zero-copy).
- Add tests alongside the code. Prefer `insta` snapshots for parse/lint output.
- If the entry needs a dependency not in `Cargo.toml`, add it (Cargo.toml is
  writable) — prefer the crates sanctioned in `CLAUDE.md`, "Tech stack".
- If the entry's `per` cite is ambiguous or rests on an unsettled decision, do
  NOT guess: leave it and surface the question (the harness will route it).

# FRICTION / REFACTOR (optional — most ticks file nothing)

Hit real friction this tick, or touched structural debt you can't fix now?
Use the `capture-friction` skill — filenames `build-<slug>.md`, committed
with your work; target directory per capture type (its own trigger condition
covers when to reach for it).

# OUTPUT

**Your checkout is an isolated worktree, and `pwd` is its root.** Every
repo-relative path in the entry resolves against it. Absolute paths are
constructed from `pwd` output only — never from a path seen in a file,
an error message, or `git worktree list`; the main checkout's path is
not yours, and one write there corrupts trunk state outside your gates.

One commit on this worktree's branch, prefixed `build:`. Imperative subject; the
body explains *why*, not a restatement of the spec. **Your branch is the only
place you commit.** Never rebase onto or merge from `main`, never push a trunk
ref, never `cd` to the root checkout to commit — if `main` has moved since your
worktree was created, ignore it; reconciling is the dispatcher's job, and a
commit made anywhere but this branch bypasses the gates and is lost to ship
bookkeeping. **Verify the target before every commit**: `git rev-parse
--abbrev-ref HEAD` must name this worktree's `flume/<tag>` branch — on a
mismatch, stop and report it as a blocker instead of committing; a
wrong-branch commit is silently lost, and retrying it burns the tick.
The same discipline binds **every file write**: absolute paths must live
under this worktree's root, never the root checkout — a root-checkout
edit is invisible to your branch and lands as stray drift for someone
else to clean up.

Gates run automatically after your commit: `cargo fmt --check` (afterCommit),
then `cargo clippy -D warnings`, `cargo test`, and `pnpm --dir sdk test`
(afterMerge). A gate failure reverts your commit and the entry returns to
pending.

**Iterate to green before you commit — this is the job, not an afterthought.**
Loop: make the change → run the gate commands (`CLAUDE.md`, "Common commands":
the cargo fmt/clippy/test trio, plus `pnpm --dir sdk test` when the entry
touches `sdk/**` — cargo says nothing about TypeScript) → if anything is red
(including an *existing* test your change broke), fix it and run again.
Repeat until fully green, then commit. **Never commit red. Never end the tick
with no commit just because the change rippled into other tests — repair
them; that ripple is part of the entry.** If you genuinely cannot reach green
(a real blocker, not just more work), do NOT bail silently: state in your final
message exactly what blocked you and what you tried, so plan can re-scope or a
human can step in. A silent no-commit is the one outcome to avoid.

Do NOT touch `.flume/plan/pending.json` — the harness updates it post-merge.
