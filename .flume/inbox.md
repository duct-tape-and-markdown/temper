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
- observed at 1408cc3d — five field reports from `dtmd-temper.md`, routed
  direct by John 2026-08-26; the operator pasted the full file the same day,
  so the detail below is complete — nothing further to pull.
  (1) tap sink resolves from cwd: **half shipped.** The worktree-loss half
  landed in 0.0.15 (`src/tap.rs::log_path` commondir hoist — file-based,
  works once GIVEN the root; the missing piece is resolving the root from
  cwd). The cwd half is real at HEAD: `temper tap [PATH]` defaults to `"."`
  (src/main.rs:146) and the SDK synthesizes the hook command bare
  (`TAP_COMMAND`, sdk/src/declarations.ts:747; this repo's projection
  .claude/settings.json:7,27), where the sibling hooks pass `.` explicitly.
  Field repro on 0.0.15: one session split 15 events at root / 1 stranded
  under a `.claude/worktrees/...` subpath; another 32/7 with the stray
  outside every tree in play — cwd at hook-fire time, not root ambiguity.
  Cost: a fanout worktree’s sink dies with the worktree (every build-tick’s
  telemetry vanishes; only the plan singleton taps land), plus stray
  `.temper/` dirs scattered in consumer source trees. Ask, in precedence:
  explicit `PATH` verbatim → git-common-dir anchor (parent →
  `.temper/tap.jsonl`) → cwd only outside a git tree; secondary, SDK emits
  `temper tap "$CLAUDE_PROJECT_DIR"` (or `.`) to match its siblings.
  Caveats: anchoring collapses N per-worktree sinks into one contended
  append — O_APPEND line-atomicity holds on POSIX, verify win32, add an
  explicit parallel-wave test; consumers reading a worktree-local tap
  silently switch to the primary — release note, not just a patch.
  Nice-to-have: a startup check surfacing stray `tap.jsonl` below the
  anchor.
  (2) tap record schema v2 — one version bump, three additions. Identities
  are raw: rules carry the machine-local absolute path (one rule split
  across 40+ identity strings over checkout/worktree/per-fanout-job roots;
  reproduced in this repo’s own tap.jsonl), and skills carry the caller’s
  raw spelling — `runner` (231 events) and `runner:runner` (88) coexist
  interleaved with no session holding both, refuting a rename-migration
  read — with no project|personal|plugin scope marker, so “unused in this
  repo” is inexpressible from the log. And no record shape carries any time
  field: append order is the only ordering proxy and dies on
  rotation/concat/second-writer. Ask: (a) canonical identity ALONGSIDE the
  raw one, never replacing it — rules: repo-relative id resolved against
  the git common dir; skills: the resolved `plugin:skill` id plus a `scope`
  field; (b) ISO-8601 UTC `ts` on every record; (c) `version: 2` signalling
  both. Caveats: consumers (the telemetry verifier included) key on
  identity — additive only; win32 backslash/drive-letter paths normalize
  carefully; whether worktree copies SHOULD collapse is a judgment — emit
  both ids and let the reader choose.
  (3) InstructionsLoaded taps only under `path_glob_match`: **confirmed at
  HEAD** (`TELEMETRY_EVENT_HOOKS`, sdk/src/builtins.ts:1620 — a deliberate
  scoping per its doc comment, now a correctness hazard). The report’s
  gating external fact is settled: Claude Code fires InstructionsLoaded for
  ALL CLAUDE.md/rules loads; documented `load_reason` matcher values are
  `session_start`, `nested_traversal`, `path_glob_match`, `include`,
  `compact` [source: code.claude.com/docs/en/hooks (retrieved 2026-08-26)].
  An always-on rule therefore loads under `session_start`, and the
  preferred fix (a) is possible: register the remaining reasons (rows or a
  wildcard matcher) — the tap already writes `load_reason` verbatim
  (src/builtin_kind.rs:710), so each load banks its own reason code.
  Stakes: a coverage audit reading the log scores always-on rules 0% and
  nominates the harness’s most load-bearing rules for deletion. Caveat:
  `session_start` fires every session for every always-on member — volume
  grows, informative only because the reason rides each record.
  (4) `coverage.checked` prints `(0)` for embedded-locus kinds. The engine
  evaluates correctly — the report verified a deliberately-false `count`
  clause on an embedded kind enumerates all members, and
  `extent`/`max_len`/`required` decide per rendered member — so the defect
  is report legibility: `member_counts` is file-discovery only
  (src/gate.rs:179, :254) while a lock can carry hundreds of nested members
  (observed: 269 `declaration.nested_member` rows across 9 embedded kinds,
  every one printing `(0)`), which reads as “this kind is dead” and cost a
  dedicated investigation to disprove. Ask, filer’s preference: report
  embedded counts with a marker — `directive (36 embedded)` beside
  `skill (16)`; omitting embedded kinds from the line is the fallback.
  Cosmetic severity, named so it doesn’t silently sink behind the batch.
  (5) duplicate `collectionAddress` unions selections and cross-applies
  contracts — **currency caveat up front**: the triggering condition no
  longer exists in the reporting checkout and the read-only re-verify did
  not reproduce on current temper; frame as confirm-and-regression-test,
  not fix-this. Original repro (0.0.14): a second fields-only kind declared
  at a built-in’s address (`settings.json` / `hooks.<Event>`) was accepted
  at declare and emit; at check both kinds’ selections unioned (6 real
  handlers → both report 12), contracts cross-applied (the new kind’s
  `maxLen` indicted the project’s own guard hooks), findings duplicated per
  kind, and the actual offender passed clean. Ask: reject a duplicate
  `collectionAddress` at declaration with a named error (a hook entry has
  no per-entry discriminator, so rejection is the cheaper, more temper-ish
  fix than read-side disambiguation) — and add the regression test at this
  address either way; the test is most of the issue’s remaining value. The
  report pairs (4) and (5) as one signal — the address/kind model is
  under-exercised — arguing for tests at the model level, not one address.
