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

## State of the era (2026-07-04)

- **The six-noun demolition wave SHIPPED** — eight build slices in one day
  (lock declaration rows → gate on artifacts+lock → SDK six-noun recut →
  CLI collapse to eight verbs → lock-assembly-rows → manifest retire →
  KIND/PACKAGE parse retire → explain unify), net ≈ −2,600 lines, plus
  GATE-FAIL-LOUD-EMPTY-ASSEMBLY (a declared-but-unresolved assembly now
  blocks). All ruled forks encoded; `explain` carries John's bare+collision-
  error ruling. Queue quiescent; every remaining thread is human-gated.
- Corpus shadow of the dogfood ruling awaits John's hand: 00-intent's
  self-hosting finish line and 90-spec-system's confirmation recipe still
  narrate the dogfood (latent references smuggle latent code).

## Parked (pointers only)

- On John: PACKAGING-CHANNELS credentials — npm org + @temper scope (the
  SDK publish is v0.1's channel 1); marketplace/signing creds.
- `(inplace-lock-producer)` fork — narrowed by the dogfood ruling to the
  external-user question only (who compiles an in-place `temper.toml` into
  lock rows until SDK emit is the producer). In open-questions.md.
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

## Goal: v0.1 release + open-source publish (set 2026-07-03)

The six-noun engine is demolished and shipped; the path is now packaging:
README stands alone, public docs speak plain language, release mechanics per
55-offering, npm creds on John. Weigh every new thread against shipping this.
