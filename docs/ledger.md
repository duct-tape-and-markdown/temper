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

## State of the era (2026-07-09 — 0019 ratified)

- **THE CENTER RECUT (0019, 6a04322)**: temper types the documents that
  program agents — a kind declares its `content` (file | layout, three
  primitives, one face: the reader); invariant 7 read-or-written-never-both;
  0018 scoped to composed hosts. The demo recut with it: this repo's spec
  corpus governing itself — the manifest fork is OFF the launch critical
  path (post-launch; its open-questions record stands).
- **META-FREEZE (John, 07-09)**: no further harness/process/audit
  investment this side of v0.1. Launch wedge is the standing session focus;
  the release workflow is the one unblocked pure-engineering item. Amended
  07-10 (0021): the manifest campaign is admitted — tag and campaign gate
  neither direction; all else stays frozen. Channel 2 first cut live 07-11:
  npx @dtmd/temper delivers prebuilt linux/win32 engines (SDK 0.0.7,
  release.yml, NPM_TOKEN repo secret); darwin + plugin channel remain with
  the entry; 0.1.0 is the tag's to stake.
- **KERNEL CORPUS LIVE** (PR #7; `metaphor-era` tag): eight nouns in
  `specs/model/`, decisions outside every read path. Flume dispatch model:
  one tick = one job, cursors in state.md, planHonestyGate. Loop economics
  re-cut 07-10: audit+sweep merged, quiet-as-job deleted, plan on Opus with
  Fable only on a live spec delta; build Opus.
- **Friction/refactor channels live**; DRAIN AT SESSION OPEN. Decision
  audit (07-09, six agents): 0001/0013/0014 gaps filed via inbox — 0013's
  note amended in place, superseded in part by 0019.
- Follow-up hygiene: src/sdk comment cites to old spec paths die on contact
  (rust.md exit clause) — incl. builtins.ts's four cites to the deleted
  `packages/*/PACKAGE.md`. (Docs/README re-cut shipped 07-10.)

## Parked (pointers only)

- SUPERSEDED by the re-cut (do not schedule): John's old ceremony batch
  (A2/C1/C2 rewords, severity two-vs-three, 10-contracts split), the
  `(code-seam-joins)` tag-grammar session, the genre-adoption pilot as
  framed (genre retired; re-frame as nested-member adoption against
  `model/representation.md` if cascade still volunteers).
- Guidance layer: 4 source-verified deltas awaiting curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED).
- Docs-language candidate (post-freeze): the determinism ladder — "push every
  check to the most deterministic layer that can express it" (field
  feedback, 07-10).
- On John: Apple Developer notarizing (decide at release); USPTO name screen.

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
demo payload is the spec corpus governing itself (0019); hooks/permissions/
MCP as members move post-launch. Weigh every new thread against shipping
this — the meta-freeze holds until the tag.
