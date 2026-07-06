# Session ledger — cross-session working state

Maintained by the interactive session assistant. One rule: this is a board
of pointers, not a narrative — design reasoning lives in the records it
links, work orders live in flume, ratified decisions live in the corpus and
are forgotten here. Target under ~60 lines, hard.

## How sessions run (ruled 2026-07-03; dogfood ruling 2026-07-04)

- temper is the product. flume is the only code path. The recursive dogfood
  is DEACTIVATED (John, 2026-07-04: cumbersome) — no `.temper/` workspace,
  no `temper.toml`, no self-check gate, no session-start hook. Validation
  lives in `tests/` fixtures; a real dogfood returns when SDK-primary
  authoring (`harness.ts` → emit) is the product's own front door.
- The interactive session designs specs with John and governs flume. It does
  not hand-execute pipeline work, even when hands are faster.
- One focus per session, picked at open; the rest of this board stays parked.
- Frame-scale changes get a cooling period: draft record → cold read → sleep
  → ceremony. No same-day ratification of new frames without John's waiver.
- Plain words over metaphor, in the corpus and here. New coinage needs John.
- Public docs voice (John 07-05): natural, no em-dashes or claude-isms, not
  pitchy. `docs/*.md` is the human-curated plain-language layer; the spec
  corpus stays the operational definition (docs defer to specs on conflict).

## State of the era (2026-07-05)

- **Front door open, first adopter live** (07-05): repo PUBLIC;
  `@dtmd/temper@0.0.2` on npm with the ratified subpath layout (PR #4 +
  SDK-RECUT shipped). cascade (John's repo) is the first external harness:
  kept `check`, reverted `install`; four field reports routed — one new
  fork `(carriage-aware-placements)` (needs John: in-place first-class +
  what carries projection/authored at the guard; session proposal: the
  lock rows carry carriage), three accepted-debt symptoms the demolition
  dissolves. John delegated finer-grained calls to the session (07-05) —
  rule, encode, he ratifies by PR merge.
- **NEXT SESSION FOCUS: the GATE-READ-LOCK-DEMOLITION decomposition
  ceremony** — serialized chain over ~18 test files + main/import/drift/
  compose, with cascade's DATUM evidence and the carriage fork as rider.
  Frame-scale: cooling discipline applies unless John waives.
- Shipped: the six-noun demolition wave (eight slices, ≈ −2,600 lines),
  `(inplace-lock-producer)` resolved (emit is the sole lock producer,
  20-surface), DRIFT-DIFF-RETIRE (−391 lines; plan's reconcile found it the
  only decoupled cut — the rest is live gate scaffolding until the rewrite).
- Corpus shadow awaiting John's hand: 00-intent's self-hosting finish line +
  90-spec-system's confirmation recipe still narrate the dogfood; the
  corpus's `@temper/claude-code` spelling awaits the `@dtmd` name
  confirmation (50-distribution, 10-contracts, 15-kinds).

## Parked (pointers only)

- On John: the rest of PACKAGING-CHANNELS release setup — marketplace/
  signing creds; the USPTO name screen before launch. (npm DONE 07-05:
  `@dtmd/temper` published, NPM_TOKEN local in gitignored `.env`.)
- `(local-overrides)` fork — no spelling for personal overrides in the
  one-value assembly model. Blocks nothing.
- Genre-adoption pilot: needs `(genre-fence-format)` (deferred to the pilot
  itself; TOML prior unbound). Hold.
- Guidance layer: 4 source-verified deltas awaiting John's curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED).
- `(code-seam-joins)` tag-grammar session: partially superseded by the
  ceremony's one-reference-concept ruling — re-check before scheduling.

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task. The loop shares this tree:
  `git status` before any restore; never restore an uncommitted authored
  file; never edit tracked files while a tick runs (plan stages `git add -A`).
- Per green tick: verify the commit, fence check
  (`git show <sha> --name-only --format= | grep -cE '^(\.claude|docs|specs)/'`
  = 0), push to origin.
- flume routes build ticks to Sonnet on the preamble's `Phase: build` line —
  a startsWith on the template never fires (the runtime prepends `<harness>`).
- `cargo install --path .` after engine waves — keep the session's binary at
  trunk head before reading check output as evidence.
- `cargo-insta` is not installed: accept snapshots with `mv .snap.new .snap`.
- Curated std-lib files (`kinds/`, `packages/`) are compiled-in embeds:
  parse slice first, curated line after.

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

The open-source publish is DONE: repo public, docs layer + README voice pass
shipped, topics set, vuln reporting on, awesome-list ≥1-week clock started
07-05. Remaining is the launch gate per 55-offering: prebuilt binaries on
three OSes, stranger-proof quickstart, regenerable demo, npm on John, USPTO
screen before launch. Weigh every new thread against shipping this.
