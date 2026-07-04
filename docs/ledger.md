# Session ledger — cross-session working state

Maintained by the interactive session assistant. One rule: this is a board
of pointers, not a narrative — design reasoning lives in the records it
links, work orders live in flume, ratified decisions live in the corpus and
are forgotten here. Target under ~60 lines, hard.

## How sessions run (ruled 2026-07-03)

- temper is the product. flume is the only code path. The dogfood validates
  finished versions — off during engine work, one confirmation pass at wave
  end (rebuild, re-import, check, commit, re-arm the self-gate).
- The interactive session designs specs with John and governs flume. It does
  not hand-execute pipeline work, even when hands are faster.
- One focus per session, picked at open; the rest of this board stays parked.
- Frame-scale changes get a cooling period: draft record → cold read → sleep
  → ceremony. (2026-07-04: John waived the sleep for the six-noun ceremony,
  explicitly.) No same-day ratification of new frames without that waiver.
- Plain words over metaphor, in the corpus and here. New coinage needs John.

## The one open design thread

- **The six-noun model: RATIFIED + ceremony ADMINISTERED** (2026-07-04,
  middle-out session; record: claude.ai/code/artifact/
  417f8f03-19d9-4036-a200-30e27b0f78db, v2 + posture-equality; cold read
  confirmed, sleep waived). The architecture corpus (10/15/20/40/45/50) was
  re-cut on branch `claude/code-smells-review-v8exig` — pre-state = main
  @ 33b960c; John's PR review of that branch IS the remaining ratification
  gate. New forks + ceremony bugs filed in `.flume/plan/open-questions.md`
  ("The six-noun model ceremony"). NEXT after merge: plan re-reconciles the
  whole pending queue against the new corpus (AGENT/COMMAND/SETTINGS-KIND
  rest on retired KIND.md mechanics); then the demolition wave, evidence-
  gated per entry.
- Intent deltas await John's hand (session drafts nothing in specs/intent/):
  05-model's concept table predates the package dissolution and the
  registration/posture vocabulary; 00-intent's Decision paragraphs still
  name KIND.md-era mechanics in passing. Laws themselves: unchanged.

## Parked (pointers only)

- `(engine-language)` fork — evidence gathered (binary ~1ms vs node ~26-178ms
  vs npx ~320-730ms at the hook placement); session recommends keep-Rust;
  John rules. In open-questions.md.
- Genre-adoption pilot: now shaped by the postures ruling — needs
  `(genre-fence-format)` first. Hold.
- Guidance layer: 4 source-verified deltas awaiting John's curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- On John: PACKAGING-CHANNELS credentials; npm publish name/scope (now also
  the @temper scope for the provider module).
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED —
  needs an empirical parser test). Guard-hook item graduated to the
  `(guard-posture)` fork.
- `(code-seam-joins)` tag-grammar design session: superseded in part by the
  ceremony's one-reference-concept ruling — re-check before scheduling.

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task. The loop shares this tree:
  `git status` before any restore; never `git restore` an uncommitted
  authored file.
- Per green tick: verify the commit, fence check
  (`git show <sha> --name-only --format= | grep -cE '^(\.temper|\.claude|docs|specs)/'`
  = 0), push to origin.
- `cargo install --path .` after engine waves — a stale PATH binary
  false-blocks the session-start gate.
- `cargo-insta` is not installed: accept snapshots with `mv .snap.new .snap`.
- Curated files are compiled-in embeds: parse slice first, curated line after.

## Goal: v0.1 release + open-source publish (set 2026-07-03)

The six-noun re-cut IS the v0.1 path: a smaller product ships sooner. README
stands alone; public docs speak plain language; release mechanics per
55-offering. Weigh every new thread against shipping this.
