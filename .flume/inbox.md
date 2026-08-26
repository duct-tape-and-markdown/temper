<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- observed at 9b289371 — field report `tap-sink-worktree-loss.md`
  (centercode-platform feedback intake, filed 2026-07-28) requests `temper tap`
  hoist its default sink to the primary checkout's `.temper/tap.jsonl` in
  linked worktrees, via `git rev-parse --git-common-dir`. **Triage: already
  shipped.** The hoist landed 6e94a6c9 → b9ea1a2f → 8b057aba (2026-07-25/26),
  driven by the prior report `TAP-WORKTREE-GIT-AWARE` (same consumer,
  2026-07-25; see decision 0046's context), and is in the published v0.0.15
  (tagged 2026-07-27). Shipped resolution is file-based (`.git` file → `gitdir`
  → `commondir`, `src/tap.rs::log_path`), no git subprocess — the report's
  git ≥ 2.31 floor concern is moot, and the hoist works with git absent.
  Consumer action is a binary update to ≥ 0.0.15; no temper change for the
  headline behavior.
- observed at 9b289371 — two residual deltas between that report's requested
  contract and shipped tap behavior, surfaced for plan to judge, not built:
  (1) explicit `[PATH]` is **not** verbatim — `log_path` hoists
  unconditionally, where the report wants an explicit argument to skip git
  resolution as the override; recommend keeping shipped behavior (writer and
  reader share `log_path`, so a verifier reading from a worktree checkout
  finds the same sink the writer used — an explicit-path escape hatch splits
  that) unless a field case needs the override. (2) a hoisted append
  `create_dir_all`s the primary's `.temper/` when absent, where the report
  asks record-nothing-exit-0; shipped behavior matches the non-hoisted
  precedent (tap has always created the workspace dir on append) — recommend
  keeping. Exit-zero totality holds in every path either way.
- observed 2026-08-10 (cascade adoption, engine 0.0.15) — `temper guard`
  denies edits to the MEMORY SOURCE file, not only the projection: with
  `mode: "block"`, an Edit targeting `.temper/memory/CLAUDE.md` (the
  module-adjacent authored home `file()` reads) exits 2 with the
  managed-projection message. Reproduces in THIS repo's own harness
  (`echo '{"tool_name":"Edit","tool_input":{"file_path":".../.temper/memory/CLAUDE.md"}}' | temper guard .`
  → exit 2), so the dogfood cannot edit its own memory source through
  Claude Code. Cause shape: the memory kind's any-depth governs glob
  (`**/CLAUDE.md`) catches the source; the guard appears to test kind
  governs-globs where the projection set should be lock-declared projection
  PATHS (rule sources under `.temper/rules/*.md` are correctly allowed —
  their kind's glob is `.claude/rules/*.md`). Consumer workaround: name the
  source outside the glob (cascade renamed to `CLAUDE.source.md`).
- observed 2026-08-10 (cascade composed-spec adoption, engine 0.0.15) — two
  findings from the typed-bodies probe wave:
  (1) `check` does not resolve a host-scoped EMBEDDED address
  (`<host-member>/domain/<key>`-shaped mention target) against the discovered
  corpus: emit resolves it (and refuses a bogus key), check reports
  `graph.route` as an ERROR and exits 1 — reproduced with the embedded kind's
  render hook removed so the projection carries real `member.domain` fences.
  Severity is engine-owned, so a consumer cannot tier it advisory; cascade's
  `Serves` edge (spec → domain member of 01-domain-map) ships as rendered
  prose until this resolves.
  (2) this repo's own `.temper/rules/pending-entry.md` states a colliding
  fanout entry "conflicts at merge and reverts the wave" — stale against
  flume 0.11 (Dispatcher.js ~1131-1151: cherry-pick conflict aborts, records
  `cherry-pick-conflict`, entry stays pending; only the offending commit is
  excluded — the wave lands). Caught when cascade's rule draft imported the
  claim and an adversarial verifier read the Dispatcher source.
- observed at 1408cc3d — five field reports from `dtmd-temper.md` (consumer
  feedback intake on the operator's other machine; routed direct by John,
  2026-08-26). The source file is NOT on this machine — entries below that
  need its detail say so; pull it before building those.
  (1) tap sink resolves from cwd: **half shipped.** The worktree-loss half
  landed in 0.0.15 (`src/tap.rs::log_path` commondir hoist). The cwd half is
  real at HEAD: `temper tap [PATH]` defaults `path` to `"."`
  (src/main.rs:146) and the SDK-synthesized hook command is bare `temper
  tap` (`TAP_COMMAND`, sdk/src/declarations.ts:747), so a hook inheriting a
  subdirectory cwd sprouts a stray `<subdir>/.temper/tap.jsonl` — field
  incident 2026-08-19, stray dir matching the session shell's cwd to the
  minute. Fix shape to judge: resolve the harness root by walking up from
  cwd (the discovery walk's precedent), and/or anchor `TAP_COMMAND` with
  `"$CLAUDE_PROJECT_DIR"`; this repo's own projection carries the vulnerable
  bare shape (.claude/settings.json:7,27).
  (2) tap record schema v2 — canonical identities + timestamp. Locally
  grounded motivation: v1 records carry no timestamp, and identity is the
  absolute path the hook payload hands over — this repo's own tap.jsonl
  holds the same rule as `/home/john-work/repos/temper/...` and as
  `~/.cache/flume-worktrees/temper/<entry>/...`, defeating cross-worktree
  aggregation. The report's full requested contract lives in dtmd-temper.md
  — pull before building.
  (3) InstructionsLoaded only registers under `path_glob_match`:
  **confirmed at HEAD.** `TELEMETRY_EVENT_HOOKS` hard-codes the matcher
  (sdk/src/builtins.ts:1620), so unconditional rule loads (collaboration.md)
  and memory loads never reach the tap; the tap classifier itself is
  reason-agnostic (src/builtin_kind.rs:708 records any `load_reason`). Fix
  site is the SDK table row, then re-emit.
  (4) coverage.checked prints 0 for embedded-locus kinds. Mechanism sketch,
  UNVERIFIED: `member_counts` is filled from the dispatch loops'
  file-discovered features (src/gate.rs:179, :254) while embedded-locus
  members ride the separate embedded extraction
  (src/compose.rs::embedded_features_by_kind) — plan verifies which join is
  missing against an embedded-kind fixture (the consumer testbed exercises
  9 embedded kinds — the ledger's centercode note — so the symptom is
  cheaply reproducible there or in a local fixture).
  (5) duplicate collectionAddress silently unions selections — filed with
  the report's own currency caveat: possibly stale against current HEAD, not
  locally verified. Detail lives in dtmd-temper.md; verify before building.
