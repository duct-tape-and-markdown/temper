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
  engines; 0.0.18 cut 2026-09-08 (tag c60c976a, smoke green). The cut
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

## Next session's one focus (11-26, loop stopped)

- **State:** 80 entries shipped since v0.0.25 (net code about +7k), loop
  stopped with the stop flag after the typecheck entry (10021f1f). Restart:
  `rm .flume/stop`, then the usual launch.
- **Queue on restart:** 3 pending entries and 16 inbox records. Most are the
  census triage (fixed checks become clauses, the install report, the
  no-lock guard, the script-verifier edge, nine review-only pins), the
  restored intent (per-member mode, layer slots, loud nondeterminism, typed
  kind identity, typecheck before the program, the unrepresented modeline,
  `package.json` never rewritten), memory claiming `AGENTS.md`, and nested
  output styles. Questions: `cross-source-edge-correlation` (on the
  reporter) and `out-of-corpus-edge-targets` (new, unread).
- **0.0.26:** main carries extension, the emitted reference, guidance
  delivery, edge intent, review verifiers, the tool kind, output styles and
  the probe fixes. Breaks: emit's declared typecheck, and binaries folded
  into tools. Cut when the queue settles; record net growth in the commit.
- **Spec census (0105):** 448 claims, 310 held, 87 review-only, 36 the code
  does not meet, 15 unclear. Ruled: the spec states intent and conformance
  is the loop's job. The census tables were in the session scratchpad
  (volatile); the routed records carry each finding.
- **Open for John:** self-hosting, meaning this harness declares the 0093
  reference skill so "the plugin a stranger installs gates this repo"
  holds; and the target state, a temper-typed spec whose invariants carry
  holders that `check` reads.
- **Flume issues filed:** #23-#28 (judge blind spots, environment-fault
  quarantine, an event stream, debt on disk, a derive reconcile mode;
  #25 closed as `flume exclusive`).

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
- On John, later: revisit the README and public-facing docs against today's model (0090-0097: extension, the emitted reference, guidance delivery); a public gist on the project once it is stable, for outside interest.
- Parked, John's call (2026-11-25): is the Rust engine worth its second implementation of the semantics? emit compiles a TS program that holds the whole truth, erases it to TOML, and Rust reimplements its meaning; every feature lands on both sides. Keep the lock as interchange either way: it is reviewable, joinable, and never runs foreign code. Evidence to gather: guard latency of a compiled-TS binary (Bun, or Node single-executable) against Rust on a real tool call; the share of entries since 0.0.20 that touched both sides; whether discovery's whole-tree walk needs Rust's speed. Full Rust is the third option, judged weakest: the TS authoring surface (typed at the keystroke, composed as ordinary code, npm-distributed, the language agents write well) is the product's front door.
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
