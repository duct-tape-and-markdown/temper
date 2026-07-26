<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

## `temper tap` resolves cwd-relative; worktree-phase records die with the worktree — observed at 82f9c9b9

Field report from centercode-platform, 2026-07-25; mechanism re-verified here
at source. `Command::Tap` carries `#[arg(default_value = ".")]` and the arm
does `path.join(WORKSPACE_DIR)` (src/main.rs:147, :393) — no upward walk, no
git awareness anywhere in src/. The SDK's canonical hook row is bare
`temper tap` (sdk/src/declarations.ts:747), so cwd wins. A Claude Code hook
firing with cwd inside a linked git worktree writes to
`<worktree>/.temper/tap.jsonl` — gitignored, so untracked — and the runner
that owns the worktree lifecycle deletes it with the worktree. The tap is
advisory and always exits zero: nothing warns.

Status: latent in the reporting repo (its worktree phase invokes no skills and
no path-gated rule covers what it touches). **Likely active in this repo since
d1ce7fcc/82f9c9b9**: our build phase runs in per-entry worktrees, our tap
hooks are the canonical projected rows, and `rust.md` is paths-scoped to
`src/**` — a build agent's rule load fires InstructionsLoaded inside the
worktree. Our own telemetry verifier (`context-arrives`) is judged from a log
that silently excludes the autonomous phase.

Ordering trap (from the report, real): any consumer-side change that makes a
worktree phase start invoking skills converts latent to active. The product
fix wants to land first.

Recommended resolution — resolve the tap's workspace git-aware: when
`<path>/.git` is a **file** (the linked-worktree marker), follow it to the
primary checkout's root and append there; hooks stay bare `temper tap`
untouched. Prefer parsing the `.git`-file gitdir pointer over shelling to
`git rev-parse --git-common-dir` — keeps the no-subprocess posture. The
`.git`-file and `commondir` layout are an external fact: cite
gitrepository-layout(5) before encoding.

Why this and not the report's alternatives: the log's own doc comment declares
it **per-machine** (src/tap.rs:26) — its identity is machine+project, and
per-checkout homing is an implementation accident that the worktree lifecycle
turns into deletion. Warn-once still loses the data and hook stderr is
effectively invisible. Per-worktree logs + runner-side drain distributes the
obligation to every runner that owns a worktree lifecycle — N fixes for one
defect, and flume-the-runner load-bearing for temper's correctness.

Objection weighed: a worktree on a divergent branch writes records referencing
a graph the primary doesn't have. Already an in-band, tolerated condition —
`read_log` counts and tolerates older-version records (src/tap.rs:236), and
the field strand joins through the current corpus as evidence, never a gate
(src/read.rs:305).

Separable meta-question, not this entry: an advisory subsystem that always
exits zero has no channel to report its own output being discarded — same
under-report-while-presenting-healthy shape as the two dropped-edge graph
defects. Wants its own decision.

