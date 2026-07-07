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

## State of the era (2026-07-06, evening)

- **KERNEL RE-CUT STAGED — awaiting John's ceremony.** The corpus is
  rewritten in the working tree: 722 lines replace 2,624; eight nouns, two
  layers; decisions evicted to `specs/decisions/` (ADRs 0001, 0002).
  Ceremony: `git tag metaphor-era` (pre-state), cold read the diff, one
  `specs:` commit. Riding for ratification inside it: CLEAN SLATE encoding
  (`distribution.md`), the `include` rename (was prose "embed"), John's
  thesis (realized as the kernel itself — 0001).
- **FLUME FROZEN — do not tick.** `.flume/prompts/*`, `PROTOCOL.md`,
  `chain.ts`, and every pending entry cite retired spec paths. Leg 3 after
  the ceremony: re-derive the queue against the new corpus; 0001's
  consequences section lists the engine work (embedded locus, one edge
  enumeration, requirement-as-kind, satisfier-set bug, hooks/permissions/
  MCP kinds); LOCK-CLAUSE-CHANNELS and the kind-flatten survive in spirit.
- Follow-up hygiene, post-ceremony: docs/README re-cut onto kernel nouns
  (stale paths in docs/how-it-works, cli, example-config, horizons,
  README); 37 src/sdk comment cites die on contact (rust.md exit clause).

## Parked (pointers only)

- SUPERSEDED by the re-cut (do not schedule): John's old ceremony batch
  (A2/C1/C2 rewords, severity two-vs-three, 10-contracts split), the
  `(code-seam-joins)` tag-grammar session, the genre-adoption pilot as
  framed (genre retired; re-frame as nested-member adoption against
  `model/representation.md` if cascade still volunteers).
- Guidance layer: 4 source-verified deltas awaiting curation —
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8.
- Verify queue: trailing-period @import (cascade CLAUDE.md:26, UNVERIFIED).
- On John: Apple Developer notarizing (decide at release); USPTO name screen.

## Standing discipline (mechanical, paid for)

- Wake-then-loop as its own background task; `git status` before any
  restore; never edit tracked files while a tick runs (plan stages -A).
- Per green tick: verify commit, fence check (`git show <sha> --name-only
  --format= | grep -cE '^(\.claude|docs|specs)/'` = 0), push to origin.
- flume routes build ticks to Sonnet on the preamble's `Phase: build` line.
- `cargo install --path .` after engine waves; `cargo-insta` absent — accept
  snapshots with `mv .snap.new .snap`.

## Goal: v0.1 release (set 2026-07-03; repo PUBLIC 2026-07-05)

Launch gate per `specs/distribution.md`: prebuilt binaries on three OSes,
stranger-proof quickstart, regenerable demo, USPTO screen on John. The
kernel's forward work (hooks/permissions/MCP as members) is the demo
payload. Weigh every new thread against shipping this.
