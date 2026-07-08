# Session ledger — cross-session parking lot

Maintained by the interactive session assistant; read on demand (session
open to pick the one focus, resuming parked work), never imported. One
rule: this is a board of pointers, not a narrative — design reasoning
lives in the records it links, work orders live in flume, ratified
decisions live in the corpus, session conduct lives in `.claude/rules/`,
and all of those are forgotten here once homed. Target under ~60 lines,
hard.

## Held rulings — no home yet (relocation needs John)

- Public docs voice (John 07-05): natural, no em-dashes or claude-isms, not
  pitchy; docs defer to specs on conflict. Candidate home: a `paths:`-scoped
  rule over `docs/**`. Use it for the docs re-cut below.

## State of the era (2026-07-06, night)

- **KERNEL CORPUS LIVE** (PR #7 merged; `metaphor-era` tag = pre-state).
  Eight nouns in `specs/model/`; decisions in `specs/decisions/`, outside
  every read path. Ratified inside it: CLEAN SLATE, the `include` rename,
  enforcement *mode* (cold-read catch), John's thesis as the kernel (0001).
- **FLUME UNFROZEN — dispatch model live** (6850c35): one tick = one job
  (inbox → spec delta → ship audit → residue → quiet), per-input cursors in
  state.md (seeded at 813ca61), planHonestyGate on the marker, decisions/
  excluded from the corpus inline. The recut derives via the delta window;
  0001's consequences list is the chain seed; inbox carries the routing
  notes (kernel-resolved forks, PACKAGING cite move).
- **Friction channel live** (`.flume/friction/`): agent→human harness
  feedback, one uniquely-named file per capture, exceptional never ritual
  (bar in its README). DRAIN AT SESSION OPEN: triage, implement or route
  (chore(harness) / chore(flume) / inbox), delete the file. Day-one yield:
  a livelock fix, the entry reference-resolution gate, two inbox items.
- Dogfood evidence (for its return case): pending entries are harness
  artifacts with dangling-reference failure modes — two build frictions
  traced to entries citing surfaces that don't exist. Entries-as-members
  under an edge-resolution clause is the product answering its own pipeline.
- Follow-up hygiene: docs/README re-cut onto kernel nouns (stale paths in
  docs/how-it-works, cli, example-config, horizons, README); src/sdk comment
  cites to old spec paths die on contact (rust.md exit clause) — now incl.
  builtins.ts's four cites to the deleted `packages/*/PACKAGE.md`.

## Parked (pointers only)

- SUPERSEDED by the re-cut (do not schedule): John's old ceremony batch
  (A2/C1/C2 rewords, severity two-vs-three, 10-contracts split), the
  `(code-seam-joins)` tag-grammar session, the genre-adoption pilot as
  framed (genre retired; re-frame as nested-member adoption against
  `model/representation.md` if cascade still volunteers).
- Guidance layer: 4 source-verified deltas awaiting curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED).
- On John: Apple Developer notarizing (decide at release); USPTO name screen;
  PR #18 cold read (engineering-shape spec + 6 refactor-channel seeds — hold
  the loop off the captures until merged).

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task; `git status` before any
  restore; never edit tracked files while a tick runs (plan stages -A).
- Sweep `.flume/friction/` at session open (and at wave end); prune
  `.flume/sessions/` when it swells (317MB at the 07-06 cutover).
- Per green tick: verify commit, fence check (`git show <sha> --name-only
  --format= | grep -cE '^(\.claude|docs|specs)/'` = 0), push to origin.
- `cargo install --path .` after engine waves; `cargo insta test --accept`
  for snapshot churn (cargo-insta 1.48 installed 07-07).

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

Launch gate per `specs/distribution.md`: prebuilt binaries on three OSes,
stranger-proof quickstart, regenerable demo, USPTO screen on John. The
kernel's forward work (hooks/permissions/MCP as members) is the demo
payload. Weigh every new thread against shipping this.
