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
  pitchy. `docs/*.md` is the human-curated plain-language layer; the spec
  corpus stays the operational definition (docs defer to specs on conflict).
  Candidate home: a `paths:`-scoped rule over `docs/**`.
- CLEAN SLATE (John 07-06): published pre-1.0 carries NO backward-compat
  burden — no shims, no aliases, no deprecation ceremony. Break freely
  until v0.1. Candidate home: the corpus (`55-offering.md`), via the
  `specs:` ceremony.

## State of the era (2026-07-06)

- Repo PUBLIC; `@dtmd/temper@0.0.2` on npm (subpath layout, PR #4). cascade
  is the first external harness; its field evidence drove the front door.
  Finer-grained calls delegated to the session — rule, encode, John
  ratifies by PR merge.
- **The front door is RATIFIED and the chain is FILED** (PR #6 merged
  0c11135; cooling waived): install absorbs init behind one question,
  depth emergent, placements follow the lock, built-in lock = embedded
  defaults, temper.toml retired as a filename.
  `(carriage-aware-placements)` resolved. The decomposition ceremony ran
  in-session; the serialized S1–S7 chain is in the inbox for plan
  (emit-payload-seam → check-reads-lock → fixtures → scratch/codec retire
  → install front door → temper.toml zero).
- **Cascade volunteered as the genre-adoption pilot** — `(genre-fence-
  format)`'s first consumer, real Decision fixtures, workshop with John
  next session. Rides emit's fence render (post-S1). Prep is staged:
  claude.ai/code/artifact/8fd56c9c-747f-4e53-8cae-e51bbf79a8a7 (4 fixtures,
  inventory, 7 questions; note: "felt-occupant" is prose, not a Decision).
- Corpus shadow awaiting John's hand: 00-intent's self-hosting finish line +
  90-spec-system's confirmation recipe still narrate the dogfood.

## Parked (pointers only)

- Six-slice ultracode audit (07-06): 32 verified findings, 4 action tracks —
  claude.ai/code/artifact/b967dc6c-5d37-411c-8f43-2d4613ec632b. **HELD by
  John: 07-06 decisions haven't derived into plan work yet — no entries or
  ceremony batches from it until plan's routing is observed.** Watch the
  next plan ticks for A1 (package entry), A3 (dial residue), C3 (sdk/
  recut); if plan reconciles without filing them, the residue-paragraph →
  entry routing is the leak to fix. One open question rides it (B2): where
  does the authored authority posture live in the four-field assembly —
  sdk emits `authority:"shared"`, a value the corpus never coined.
- John's thesis (07-06, unencoded): the findings cascade toward a core
  design simpler than it seems. Test for each collapse: does the merge
  delete ceremony (role/requirement precedent — do it) or delete sayable
  postures (required/count — refuse it). Clarified posture: generalize as
  far as possible; ship deliberate code as instances of the generic — "the
  first party is a user." Deliberate code inside the engine = a missing
  generic capability, not a licensed exception. Bounds held: the definition
  stays un-authorable (law 3); generalize on consumption, never in advance
  (entry gate); diagnostics keep instance vocabulary (debug labels).
  Candidate home when encoding resumes: one sentence in 00-intent's "One
  engine, every layer an instance."
- Severity two-vs-three: clause severity is `required`/`advisory`; the
  surface-authority posture (20-surface) speaks note/warn/block, with
  `note` an information tier the algebra lacks. Rule: three-valued
  everywhere, or `note` is not a severity. Surfaced by the 07-06 layering
  audit; small ceremony, do before reporters multiply.
- 10-contracts split (housekeeping): 480+ lines, arguably two topics
  (clause algebra; requirements+admissibility). Natural moment passed with
  the set-scope Decision — next corpus-wide pass.

- On John: release remainders — (a) Apple Developer membership IF we
  notarize the standalone mac binary (decide at release; npm installs dodge
  Gatekeeper); (b) USPTO name screen. Marketplace needs NO creds (verified
  2026-07-06, code.claude.com/docs: a marketplace is a git repo +
  marketplace.json; community-marketplace submission is a free web form +
  `claude plugin validate`; plugin signing does not exist). npm DONE 07-05.
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

Publish is DONE (repo public 07-05, docs layer shipped, awesome-list clock
running). Remaining is the launch gate per 55-offering: prebuilt binaries on
three OSes, stranger-proof quickstart, regenerable demo, USPTO screen on
John (marketplace: no creds needed, verified 07-06; mac notarization a
decide-at-release cost). Weigh every new thread against shipping this.
