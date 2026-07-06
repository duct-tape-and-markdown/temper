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
  rule over `docs/**`.
- CLEAN SLATE (John 07-06): pre-1.0 carries NO backward-compat burden.
  Candidate home: `55-offering.md` — ride the next ceremony batch.
- John's thesis (07-06): the design collapses toward a statable kernel;
  posture "generalize; ship deliberate code as instances — the first party
  is a user." Bounds: definition un-authorable; generalize on consumption;
  diagnostics keep instance vocabulary. Collapse test: merges that delete
  ceremony (do) vs sayable postures (refuse). Candidate home: one sentence
  in 00-intent "One engine, every layer an instance."

## State of the era (2026-07-06)

- Repo PUBLIC; `@dtmd/temper@0.0.3` on npm. The S1–S7 demolition chain has
  FULLY DRAINED; the lock is the gate's sole declaration source. Package
  noun + reachability dial demolished from the engine (audit tracks A1/A3).
  DERIVED-LOCK chain (D1–D5) filed to the inbox 07-06 — plan derives next.
- Plan prompt fixed forward 07-06 (corpus sight, spec-delta window,
  unconditional residue sweep, gate re-test, cursor); open-questions drained
  1280→~110 with a prune rule in file + prompt.
- Corpus shadow awaiting John's hand: 00-intent's self-hosting finish line +
  90-spec-system's confirmation recipe still narrate the dogfood.

## Parked (pointers only)

- John's ceremony batch (text-only, one cold read): A2 GitHub-reporter
  residue sentence, C1 satisfies-resolves-to-target reword, C2 90-spec-system
  SEED sentence, severity two-vs-three, 10-contracts split (economic teeth:
  plan inlines the whole corpus every tick), CLEAN SLATE encode. Audit
  synthesis: claude.ai/code/artifact/b967dc6c-5d37-411c-8f43-2d4613ec632b
  (Track D hygiene folds into whichever entries open those files).
- Genre-adoption pilot: cascade volunteered (07-06); workshop designs the
  fence grammar vs real Decision fixtures. Prep:
  claude.ai/code/artifact/8fd56c9c-747f-4e53-8cae-e51bbf79a8a7. Priority
  rising: the decision genre is the typed home for the process-history prose
  now displaced into commit bodies (open-questions lesson, 07-06).
- On John: release remainders — Apple Developer IF notarizing (decide at
  release); USPTO name screen. Marketplace needs no creds (verified 07-06).
- Guidance layer: 4 source-verified deltas awaiting curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED).
- `(code-seam-joins)` tag-grammar session: partially superseded by the
  one-reference-concept ruling — re-check before scheduling.

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task; `git status` before any
  restore; never edit tracked files while a tick runs (plan stages -A).
- Per green tick: verify commit, fence check (`git show <sha> --name-only
  --format= | grep -cE '^(\.claude|docs|specs)/'` = 0), push to origin.
- flume routes build ticks to Sonnet on the preamble's `Phase: build` line.
- `cargo install --path .` after engine waves; `cargo-insta` absent — accept
  snapshots with `mv .snap.new .snap`.
- Curated std-lib (`kinds/`, `packages/`) are compiled-in embeds — dies at
  D5 of the derived-lock chain.

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

Launch gate per 55-offering: prebuilt binaries on three OSes, stranger-proof
quickstart, regenerable demo, USPTO screen on John. Weigh every new thread
against shipping this.
