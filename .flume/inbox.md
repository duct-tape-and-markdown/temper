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
  so the detail below is complete — nothing further to pull. An adversarial
  pass (John's direction: the reporting environment may carry its own
  misconfiguration temper should not correct for) re-verified each against
  platform docs, the spec, and this repo's own log; verdicts are inline —
  (1) and (3) strengthened into temper's own defects, (2) split into a live
  defect plus a spec collision, (4) unchanged, (5) already self-caveated.
  (1) tap sink resolves from cwd: **half shipped; adversarially verified —
  the defect is temper's, not the reporter's.** The worktree-loss half
  landed in 0.0.15 (`src/tap.rs::log_path` commondir hoist — file-based,
  works once GIVEN the root). The cwd half is real at HEAD: `temper tap
  [PATH]` defaults to `"."` (src/main.rs:146) and the SDK synthesizes the
  hook command bare (`TAP_COMMAND`, sdk/src/declarations.ts:747). The
  adversarial check settles blame: Claude Code documents that hook cwd
  follows Claude — the worktree root after entering a worktree, the new
  directory after a `cd` [source: code.claude.com/docs/en/hooks (retrieved
  2026-08-26)] — so subdir-cwd hook fires are the platform's contract and
  the cwd default is wrong against it; the reporting environment is NOT
  misconfigured. The same doc reshapes the fix: `${CLAUDE_PROJECT_DIR}` is
  documented to stay at the main checkout "regardless of the working
  directory when the hook runs", worktrees included — so the SDK emitting
  `temper tap "$CLAUDE_PROJECT_DIR"` alone closes BOTH halves
  (subdir-proof, primary-homed) at the wiring level; the binary-side
  root-walk-up is defense-in-depth for foreign wirings, and the report's
  explicit-PATH-verbatim precedence stays declined per the earlier triage
  (writer and reader must converge on one sink). The field repro on 0.0.15
  (one session split 15 events at root / 1 stranded under
  `.claude/worktrees/...`; another 32/7 with the stray outside every tree
  in play) is consistent: the hoist only sees a `.git` file at cwd itself,
  so any subdir cwd strays. Caveats kept: anchoring collapses N sinks into
  one contended append (O_APPEND line-atomic on POSIX, verify win32, test
  under a parallel wave); consumers reading a worktree-local tap silently
  switch to the primary — release note. Nice-to-have: startup sweep for
  stray `tap.jsonl` below the anchor.
  (2) tap record schema v2 — **adversarial split: one live engine defect
  (stronger than the report's claim) and one design question; do not build
  the design half as filed.** The defect, verified in this repo: the
  reader's join is exact-string through the member index
  (src/telemetry.rs:60 against `features.id` keys, src/read.rs:149) while
  InstructionsLoaded identity is the hook payload's absolute path — so NO
  rule-load record has EVER joined, any machine, worktree or not: `temper
  explain rust` narrates "No tap event in the log names it" against 31
  instructions_loaded records naming this repo's own rules, while the
  skill join (identity = member id) works (`capture-friction`: 3 of 3).
  Fix must reconcile writer identity with member id; the worktree-fork
  problem the report documents (one rule split across 40+ identity strings
  over checkout/worktree/fanout roots) folds into the same reconciliation.
  **Session recommendation (2026-08-26, John delegated the lean; encode as
  the ruling unless plan finds a collision): the file is not the API — the
  binary is.** Hold pipeline.md's category; the spec already carries the
  evolution machinery (versioned-in-lockstep, older records tolerated out
  loud) that makes a bump cheap, and flume 0.11's `log --json` is the
  family precedent for serving external demand through a read verb over a
  private file. Concretely, v2 = two changes serving temper's OWN reader:
  (a) identity becomes the repo-relative path, relativized against the
  same anchor the sink resolves to (collapsing the worktree fork as a side
  effect), raw absolute path demoted to a secondary debug field; the
  READER normalizes repo-relative → member id — it has the lock, the
  writer doesn't, keeping the tap a dumb recorder. (b) an ISO-8601 UTC
  `ts`, defended on-contract: context-arrives' failure intent ("a rule
  that fails to load") is undecidable against an undated log — "loaded 100
  times, never since the refactor" and "loads today" are identical without
  it; the field strand can narrate recency. The `scope` field is DECLINED:
  the member-index join already drops non-member records from every tally,
  so it serves only off-contract reads and freezes a drift-prone external
  taxonomy into the record. No stability promise on the JSONL; if field
  demand for tap data persists, the growth path is a `--json` read-verb
  face, never the file. The skill dual-spelling observation (`runner` vs
  `runner:runner`) is Claude Code's caller input faithfully recorded;
  write-time canonicalization stays declined for the same
  dumb-recorder reason — resolution is read-side or nowhere.
  (3) InstructionsLoaded taps only under `path_glob_match`: **confirmed at
  HEAD, and adversarially STRENGTHENED — temper's own dogfood contradicts
  its wiring.** `TELEMETRY_EVENT_HOOKS` hard-codes the matcher
  (sdk/src/builtins.ts:1620; its doc comment says the scoping was
  deliberate). But this harness's own `context-arrives` requirement
  (.temper/harness.ts) claims "every always-on member this harness
  declares should actually reach the model" with
  `verifier: telemetry(["InstructionsLoaded", "SkillInvoked"])` — evidence
  the synthesized wiring can never record, since always-on members load
  under `session_start` and the matcher filters them. The requirement is
  structurally unverifiable by its own wiring — an on-contract
  self-contradiction, not merely protection for off-contract log readers.
  The gating external fact is settled: Claude Code fires
  InstructionsLoaded for ALL CLAUDE.md/rules loads; documented
  `load_reason` values are `session_start`, `nested_traversal`,
  `path_glob_match`, `include`, `compact` [source:
  code.claude.com/docs/en/hooks (retrieved 2026-08-26)]. Fix (a) is
  possible and now obligatory for the dogfood's own claim: register the
  remaining reasons (rows or a wildcard) — the tap already writes
  `load_reason` verbatim (src/builtin_kind.rs:710). Volume caveat stands
  (session_start fires per session per always-on member), and the new
  records join nothing until (2)'s identity-join defect is fixed — (2)
  precedes or ships with (3).
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
