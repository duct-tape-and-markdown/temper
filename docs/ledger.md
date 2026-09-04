# Session ledger — cross-session parking lot

Maintained by the interactive session assistant; read on demand (session
open to pick the one focus, resuming parked work), never imported. One
rule: this is a board of pointers, not a narrative — design reasoning
lives in the records it links, work orders live in flume, ratified
decisions live in the corpus, session conduct lives in `.claude/rules/`,
and all of those are forgotten here once homed. Target under ~60 lines,
hard.

## State of the era (2026-07-16)

- **External-yield probe done (07-22)**: `temper check` on 9 real external
  harnesses. Core insight validated — caught freenet's Cursor `description`
  key (headline value prop, real repo). But surfaced 4 temper bugs the
  self-mirror can't: `check` hard-errors/aborts on missing-name + malformed
  frontmatter (4/9 harnesses crashed, reported nothing), `command` kind
  false-fails name/desc (CC makes them optional), skill-guidance on command
  findings, gate-installed noise on foreign repos. Filed `.flume/inbox.md`
  (5-8). **Redirect: robustness on foreign input outranks governing `.flume/`
  — temper must stop crashing on real harnesses before any adoption story.**
- **The center (0019)**: temper types the documents that program agents;
  the launch demo is this repo's spec corpus governing itself. Kernel
  corpus in `specs/model/`; decisions outside every read path.
- **META-FREEZE struck (John, 07-18)**: the 07-09 freeze no longer
  described a week of sanctioned harness work; its point — v0.1 ships
  before gold-plating — lives in the goal section. Loop may propose
  its own harness diffs via `.flume/amendments/` (0044,
  propose-and-ratify only); session-open sweep now covers it.
- **Distribution**: channel 2 live 07-11 — `npx @dtmd/temper` delivers
  prebuilt linux/win32 engines (SDK 0.0.11 cut 07-21, smoke-green — 0.0.10 07-19; 0.0.8-0.0.9 deprecated — mismatched pins, enablement wire split; post-publish smoke gates every cut; release.yml,
  NPM_TOKEN repo secret; the cut procedure is now encoded in the `release`
  rule, not here). Darwin + plugin channel ride PACKAGING-CHANNELS-REMAINDER
  (parked in pending.json); 0.1.0 is the tag's to stake.
- **Consumer campaign closed 07-16**: posture-recursion ruled — 0025
  (82c816e, amended cc5a9b33), prototype at
  `docs/proposals/posture-recursion/`; the built-in adoption is flume's
  as SKILL-NESTED-REFERENCE-DOCS. Open forks live in
  `.flume/plan/open-questions.md` (four; none block the queue).
- **centercode = the structural-half dogfood (07-20)**: the consumer
  testbed exercises layout/requirements/graph/degree/count/nested-docs
  hard (9 embedded kinds, one factory); the behavioral half — verifiers,
  `when`/`dial`/`extent`, local commitment — is un-field-tested and is
  the latent-bug surface (`read_dial` + `when`-guard both lived there).
  Standing direction: validate it (dogfood + adversarial passes), never
  cut valuable capability for want of a week-old consumer; lean cuts fat
  (restatement, ceremony, over-claim), not capability-ahead-of-adoption.

## Next session's one focus (John + session, 09-03)

- **Cut 0.0.16, then rule `(hook-member-identity)`.** The 09-03 field
  batch (GH #26–#38) shipped in one round: 14 entries, every ruled issue
  landed, changelog drafted under Unreleased. Cut waits on NPM_TOKEN
  rotation (set 07-19; prior token died in ~14 days). After the cut:
  `temper install` here (the new PostToolUse Bash placement and emit-side
  banners reach this repo's own `.claude/` as a `chore(harness):`), close
  #26 #27 #28 #30 #31 #33 #36 #37 #38, comment #29 #32 #34 #35. Then the
  hook fork: three candidates in open-questions, session recommends
  identity = event + matcher.

## Parked (pointers only)

- 09-03 round residue: `(external-commitment)` Decision for GH #29
  (`commitment: "external"` locus — committed, never emitted, roster
  member; session to draft). Gauntlet external-harness fixtures parked
  on John's license call. Posture rule for string path compares outside
  `src/path.rs` (class 3 of the defect review) is a harness commit, not
  filed. Derivation gap seen four times this round: plan omits insta
  `.snap` companions and constant-home files from `files[]`; a chain-side
  derivation check (entry editing a file with an insta test lists its
  snapshots) is chain territory, human commit.
- Hand-landing rule (learned 09-03): when the interactive session lands a
  build commit itself, run the chain's afterMerge list from `chain.ts`
  (`cargo clippy`, `cargo test --no-fail-fast`, `cargo doc`, `sdk test`)
  plus `cargo fmt --check` — never a remembered subset; `cargo doc` was
  the one missed. Disclose the hand-merge in the ship commit body.
- flume 0.13 adoption (one entry when it tags; flume-c7 messages this
  session): build handoff filters `quarantinedTags` and stops on
  `nothingPickable`; delete `withTickMetrics` + `metrics.jsonl` once
  verdict rows carry per-invocation usage; `blockedBy` BREAKS to
  `{kind:"blockedBy", tags:[…]}` (single-tag form gone, empty list
  refused) — migrate live entries in pending.json + `pending-entry` rule
  + plan prompt prose in the same commit as the pin bump. Three seams
  sent to flume 09-03 (capture-drain starvation, quarantine surviving a
  re-scope, killed-supervisor recovery).
- Launch loop detached (`setsid nohup … & disown`) with a Monitor on the
  log; a tool-owned background task was killed mid-merge once. `pgrep`
  for the loop must use a bracket pattern (`[c]li.js loop`) or it matches
  its own command line.

- Sweep-dock blend (John, 07-20/22): settled model — two authored
  surfaces, one product. Harness = standing law (invariants + conduct,
  sweep-agnostic; `.temper/inbox.md` is its feedback intake — testbed
  acf5919178, 52 green, tap wired). Runtime = flume-dock: briefs carry
  effort procedure + injected data; friction.md = runtime feedback; the
  loop writes the harness intake directly (`--intake`, operator-declared;
  extraction pass-through; SPEC 17-19); rubric = Remedy ∧ covers.
  Three-tier cascade ratified (07-22 research: Spec Kit/Kiro/BMAD/
  Taskmaster/aider converge): big model dictates spec, mid model derives
  atoms (entry = objective+files+acceptance+cite), cheap model executes;
  two flume seams filed to flume's inbox (per-phase model; entry-scoped
  write guard). Shakedown: sonnet wave A's 21 findings = answer key
  (scratchpad); haiku coverage fabrication now gated. Temper inbox holds
  4 engine findings (reporter mute w/ repro, managed-by placement,
  version-skew, guard semantics). Toy validation GREEN (07-22, 4 cycles,
  fixture standing at Repos/toy-sweep-target): v4 = full answer key,
  mechanical settle, self-hibernation, zero routing reverts on haiku both
  phases; fixes per cycle: one-job prohibition, routing gate + diff-step,
  coverage-format gate, <remediable> lookup (judgment→data). flume seams
  landed upstream same day and adopted (--plan-model/--build-model,
  entryChannelPaths; flume-dock rides file:../flume @0.3.1+v0.4).
  Residual candidates: tag-middle fidelity; settle could require audit
  cursor ≥ last ship; undock flushes dirty channel files. Next: clean
  twins over 9c8d78aa47..d0734c2643 (--plan-model sonnet-class,
  --build-model haiku, --intake .temper/inbox.md, tap aggregation).
  Mirror-push ruling still open before any remote-pushed wave.

- flume 0.3.1 publish (John): three 07-18 runtime fixes are LIVE via a
  patched installed dist in temper's node_modules (ephemeral — a
  reinstall wipes it) and mirrored on flume branch
  fix/worktree-escape-and-loop-lock (written against 0.2.0 source;
  reconcile with wherever 0.3.0's source lives). Fix 1: worktrees
  relocate outside the repo (FLUME_WORKTREES_DIR; temper's chain sets
  ~/.cache/flume-worktrees/<repo>) — root cause of the stray writes
  was models deriving the root checkout from the worktree path prefix.
  Fix 2: loop pidfile lock refuses a second supervisor. Fix 3: a
  merge-reverted entry's actual commit footprint persists as
  entry.observedFiles and joins the partition, so retries never ride
  with what they collided with. The wave-chaining auto-unblock rides
  the same branch.

- Guidance layer: 4 source-verified deltas awaiting curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED).
- Docs-language candidates (when docs are written): the determinism
  ladder — "push every check to the most deterministic layer that can
  express it"; the harness pin (John, 07-18) — "our job in the harness
  is to name the invariants, and let the loop settle".
- Consumer-format constants' home (parked, John 07-20): `MAX_IMPORT_HOPS`
  (src/graph.rs) is a target-format fact baked as an engine constant with
  its cite in a comment — it drifted (5 vs the real 4, fixed 14719f2).
  Considered declaring such caps on the cited kind (memory kind's
  import-directive), engine reading them through the lock; deferred — a
  lock-schema extension for a single one-format fact, and it only dedups
  the low-harm internal axis (guidance prose is advisory; the engine
  constant is the sole load-bearing home). **Trigger to revisit: a second
  import-bearing format** makes per-format declared caps non-speculative.
  Freshness (engine-vs-reality drift) is an inherent bound — temper is
  offline/decidable-only, so only re-verification catches it; not encoded.
- Multi-harness read-face spike (John agreed 07-23): declare a
  `cursor-rule` custom kind in a testbed, point `check` at a real Cursor
  repo — zero `src/` changes proves kinds-are-data; any engine change is
  a pre-0.1.0 custom-kind gap. Cheap, post-launch-weighted; the split and
  the write face's parking live in `(multi-harness-projection)`.
- Base harness dogfood: primer `docs/base-harness-primer.md`; example at
  `examples/base-harness/` (third cut shipped 549969f); built-in-kind doc
  audit at `docs/market-formats.md`. Sequencing: stranger dry run next,
  then channel 3.
- On John:
  **Rotate NPM_TOKEN** — pasted in chat (07-19, v0.0.8 rescue), treat as
  exposed; owner accepted the risk 07-21 and it published 0.0.11, so rotate
  at leisure, not urgently. New token → `gh secret set NPM_TOKEN` + `.env`.
  Note the expiry cadence: the prior token died in ~14 days and cost a
  release-day debug.
  Apple Developer notarizing (decide at release); USPTO name screen.

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task; `git status` before any
  restore. flume ≥0.12: agents run in private worktrees and merge sites
  absorb operator commits — edit and commit freely while a tick runs;
  pause via `flume stop`, never a kill.
- At session open: sweep `.flume/friction/`, `.flume/refactor/`, and
  `.flume/amendments/` (ratify or decline, then delete — 0044);
  delete `.flume/prior-attempts/` records whose entry re-scoped or
  shipped (write-only to plan — `.flume/PROTOCOL.md` has the rule);
  prune `.flume/sessions/` when it swells.
- Per green tick: verify commit, fence check (`git show <sha> --name-only
  --format= | grep -cE '^(\.claude|docs|specs)/'` = 0), push to origin.
- `cargo install --path .` after engine waves; `cargo insta test --accept`
  for snapshot churn.

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

Launch gate per `specs/distribution.md`: prebuilt binaries on three OSes,
stranger-proof quickstart, regenerable demo, USPTO screen on John. Weigh
every new thread against shipping this — the meta-freeze holds until the
tag.
