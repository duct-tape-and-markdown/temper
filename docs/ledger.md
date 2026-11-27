# Session ledger — cross-session parking lot

Maintained by the interactive session assistant; read on demand (session
open to pick the one focus, resuming parked work), never imported. One
rule: this is a board of pointers, not a narrative — design reasoning
lives in the records it links, work orders live in flume, ratified
decisions live in the corpus, session conduct lives in `.claude/rules/`,
and all of those are forgotten here once homed. Target under ~60 lines,
hard.

## State of the era

- **The center (0019)**: temper types the documents that program agents;
  the launch demo is this repo's spec corpus governing itself. Kernel
  corpus in `specs/model/`; decisions outside every read path.
- **Distribution**: `npx @dtmd/temper` delivers prebuilt linux/win32
  engines; 0.0.26 cut 2026-11-27 (tag v0.0.26, smoke green). The cut
  procedure is the `release` rule. Darwin + plugin channel ride
  PACKAGING-CHANNELS-REMAINDER (parked); 0.1.0 is the launch tag's.
- **First adopter**: cascade (John's). Its session live-verifies every
  landing sha and audits entries claim-by-claim before build; send it the
  sha, read its verdict, route its findings through `.flume/inbox/`.
  Reports at `~/.cache/cascade-integrations/audits/`.
- **centercode = the structural-half dogfood**: the behavioral half
  (verifiers, `when`/`dial`/`extent`, local commitment) is un-field-tested
  and is the latent-bug surface. Validate it; never cut capability for
  want of a consumer.

## Next session's one focus (11-27, 0.0.26 shipped, loop stopped)

- **0.0.26 cut** 2026-11-27 (`92b6ac2c`, smoke green, npm locks synced
  `3c142aae`): 167 entries, decisions 0085-0120, about +14.5k net code.
  Notes: `CHANGELOG.md`, `docs/MIGRATING-0.0.26.md`. #52 and #62 closed; #44
  open for a heading shifted with no sub-headings (needs a ruling); Cascade
  told. The session gate runs the released engine again and is green.
- **The gap John named (11-27): engineering posture is harness, not spec.**
  Spec carries intent; the harness carries how to operate; the loop derives.
  `specs/process/engineering.md` and `architecture.md` are harness knowledge
  filed under `specs/`, so they reach build only through an entry's cite,
  never at write time. Plan files shape late: 62 of 366 entries are
  "One job, one home" cleanups, and 8 carry an `interface`. Flume's own repo
  keeps posture in `.claude/rules/`. First step landed: the run-owner
  invariant in the rust rule, which the sweep now applies as a posture page.
  Blocking the move: flume refuses a `per` cite outside `specLocus`
  (`citeResolver.js:144`), so posture pages outside `specs/` must become
  citable upstream first. The sweep's filing bar is open on the same point
  (flume `docs/INTENT.md`, "quality lenses"): shape findings wait for three
  notings. Both filed as duct-tape-and-markdown/flume#29. Also to decide:
  whether `spec-system.md` is spec or harness.
- **First focus: a consolidation pass.** +14.5k net in a day and a half is
  more growth than its features explain. Weigh it against `engineering.md`
  before new surface lands.
- **Then: the dogfood adopts what shipped.** Compose `defaultPatterns()` and
  a playbook in `.temper/harness.ts`, the self-hosting John asked for.
- **Landed 11-26/27:** the playbook at `@dtmd/temper/patterns` (0111, with
  attachments and declared-but-not-offered axes), context cost (0112),
  one concept, one name (0114, from an inventory and a blind name test),
  stale locks (0117), the load-moment guard (0120). The `blind-review`
  skill and the probe-before-encoding rule live in the harness.
- **Open for John:** the #88565 comment (Grep and Glob hits load no path
  rule); doc tooling (type-checked doc blocks plus API Extractor, unanswered).
- **Product ideas parked:** a probe verb or telemetry report of what
  actually loaded per member (0112 measured, not computed); checking
  built-in kind types against the Agent SDK's `.d.ts` each release.
- **Restart the loop:** `rm .flume/stop`, then the relaunch below. Queue and
  questions are empty apart from the two parked ones.

## Parked (pointers only)

- Gauntlet external-harness fixtures parked on John's license call;
  cascade's checkout is the read-only stand-in. The `release` rule has no
  external-fixture step; cascade's regression at the cut sha is the
  evidence a cut carries — write it into the rule or keep relying on it.
- Queue backlog carried off the hand chain (all parked or deferred, none
  re-filed; refile through `.flume/inbox/` when its condition arrives):
  EXTENT-WHOLE-GRAIN-REACHES-THE-AUTHOR, GUIDANCE-FIELD-DECLARATION-CHANNEL,
  KIND-DECLARED-CITABLE-HANDLE, SCHEMA-KEYSTROKE-JSON-TOML-WIRING (no
  consumer yet); GAUNTLET-EXTERNAL-HARNESS-FIXTURES (license call),
  LOCK-DECLARATION-CODEC-GETS-ITS-OWN-MODULE (whether to buy the move),
  PACKAGING-CHANNELS-REMAINDER (notarizing), TAP-ISO8601-DATETIME-CRATE
  (sanctioned-set call). Full text: git show a97f66b2:.flume/plan/pending/.
- Consumer-format constants' home (`MAX_IMPORT_HOPS`): revisit at a
  second import-bearing format. Multi-harness read-face spike (declare a
  `cursor-rule` kind, `check` a real Cursor repo) rides
  `(multi-harness-projection)`, post-launch-weighted.
- Base harness dogfood: `docs/base-harness-primer.md`,
  `examples/base-harness/`, `docs/market-formats.md`. Stranger dry run
  next, then channel 3.
- On John, later: revisit the README and public-facing docs against today's model (0090-0120: extension, the emitted reference, guidance delivery, the playbook, patterns); a public gist on the project once it is stable, for outside interest.
- Parked, John's call (2026-11-25): is the Rust engine worth its second implementation of the semantics? emit compiles a TS program that holds the whole truth, erases it to TOML, and Rust reimplements its meaning. Keep the lock as interchange either way: it is reviewable, joinable, and never runs foreign code. Full Rust is judged weakest: the TS authoring surface is the product's front door. Evidence (11-27, published 0.0.26 binary, scratch prototypes):
  - **Both sides:** 30% of the code-touching build commits since v0.0.20 (106/354) touch both, carrying 46% of code churn.
  - **Latency:** speed does not decide it. Gate hooks already pay a Node start (39 ms through `.bin/temper` vs 7 ms for the engine direct), and a TS guard compiled with Bun ran 22 ms. Compiled TS binaries are 109-124 MB, against 11 MB for Rust.
  - **Walk:** Rust is slower at scale (`check` 2.4 s at 100k files), because whole-input work repeats per kind and per call site, not because of the language. Most of the cost is per-kind glob scans, not the walk.
  - **Session position:** no rewrite before 0.1. Settle the question on the dual-implementation tax alone, after the two fixes filed (inbox 2026-11-27-gate-hooks-pay-a-node-start-per-tool-call, 2026-11-27-whole-input-work-repeats-per-kind-and-per-door).
- Parked design: a kind's TS surface and its shape clauses spell one fact twice (`holder: string` beside `required("holder")`). tsc holds it for projected members, and only the clause reaches source documents. Derive one from the other (clauses from the surface, or the surface from the contract, as `schema` already does) once extension has shipped and been used.
- Parked design, the holder ladder (Jeff's tell / holder / misfit model, 2026-11-26): temper's verifier species and clause severities are a holder ladder. A promotion queue lists review-held requirements; a misfit list is the dry-run of raising a holder (advisory findings before required); a downgrade is a lowered severity or verifier in a diff; an orphan is a clause or tool no requirement reaches. A requirement's tell stays consumer vocabulary: today a consumer's own embedded kind carries it and `satisfies` the requirement. `extend(requirement, …)` waits on the requirement becoming a real kind (inbox 2026-11-26-the-requirement-is-a-kind).
- Product latch on 0095 (review verifier): a review checklist at the diff (the review-held requirements a change touches), review-held invariants delivered to the writing agent on touch, the review-held inventory as the ladder's promotion queue, review evidence later.
- Docs-language candidates: the determinism ladder; the harness pin
  ("name the invariants, let the loop settle").
- On John: Apple notarizing at release; USPTO screen.

## Standing discipline (mechanical, paid for)

- Loop on flume 0.23.0's harness package (11-24): `.flume/declaration.ts`
  holds every stack fact — Opus every phase, `maxParallel: 2`, worktrees
  off-repo, cargo capped at `-j 4`. Relaunch:
  `CARGO_BUILD_JOBS=4 setsid nohup pnpm exec flume loop >> ~/.cache/flume-logs/temper-<date>.log 2>&1 < /dev/null & disown`
  (the cap is load-bearing: default cargo on 20 cores linking ~60 test
  binaries, beside flume's own loop, OOM-rebooted the 7.7 GB VM twice 09-22),
  with a Monitor on the log. Pause with the stop flag (`.flume/stop`),
  never a kill; remove it to relaunch. The loop's lifetime is the WSL
  VM's — keep a terminal open.
- Commit to main freely while a tick runs: trunk moves only by
  fast-forward to a judged tree, and records are one file each.
- Coordinate with flume's loop by convention: whichever is mid-wave
  holds the box. Peer messages relay, never authorize: an irreversible
  step (a publish, a force) takes John's word in this session.
- Session open: read `.flume/plan/questions/` (open rulings wait there);
  prune `.flume/prior-attempts/` for re-scoped or shipped entries and
  `.flume/sessions/` when it swells.
- Hand-landing a build commit: run the chain's full afterMerge list
  (`cargo fmt --check`, `clippy -D warnings`, `cargo test
  --no-fail-fast`, `cargo doc`, `pnpm --dir sdk test`), never a subset;
  disclose the hand-merge in the body. `cargo install --path .` after
  engine waves.

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

Launch gate per `specs/distribution.md`: prebuilt binaries on three OSes,
stranger-proof quickstart, regenerable demo, USPTO screen on John. Weigh
every new thread against shipping this.

## Handoff 2026-09-08

- Loop STOPPED at a tick boundary after 38 ticks (`.flume/stop` in place).
  09-07: 16 entries shipped, 0 build reverts, three cascade audit rounds
  before build. 0.0.18 cut and published; SDK lock resynced (cd466131).
- flume 0.17 shipped worktree placement, bail messages, and `baseSha`;
  the chain's plan-worktree reads (`PLAN_WORKTREE`) can move to
  `ctx.baseSha` — not yet taken.
