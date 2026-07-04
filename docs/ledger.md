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
  → ceremony. No same-day ratification of new frames.
- Plain words over metaphor, in the corpus and here. New coinage needs John —
  the rung/floor/manifest reversals were metaphor connotations mistaken for
  design decisions.

## The one open design thread

- **TS-primary** ("temper is an authoring tool; the output is the harness;
  the TOML is internal"): DRAFT v2 + a five-point adversarial pass, awaiting
  John's cold read after a cooling period —
  claude.ai/code/artifact/5ef1905d-a4f1-4fd0-b553-3b3a1a9a7b1f. Unresolved
  inside it: the Node/Rust split (full enforcement needs the toolchain lane;
  the offline binary checks artifacts only). The five amendments are not yet
  folded — fold on John's word, then ceremony. Flume's queue is human-gated
  on exactly this; nothing is pickable until it lands.

## Parked (pointers only)

- Guidance layer: 4 source-verified deltas awaiting John's curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Genre-adoption pilot: unblocked (display rule shipped); its shape depends
  on the TS-primary ruling. Hold.
- Corpus de-jargoning: ride the TS-primary ceremony's deltas — never its own
  churn pass.
- On John: PACKAGING-CHANNELS credentials; npm publish name/scope.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED —
  needs an empirical parser test); the installed guard hook is advisory-only
  (always exit 0) — confirm that matches 50-distribution's intent.
- `(code-seam-joins)` wants its tag-grammar design session. The drift re-cut
  ceremony is unblocked.

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

README stands alone; public docs speak plain language (no internal
vocabulary); release mechanics per 55-offering. Weigh every new thread
against shipping this.
