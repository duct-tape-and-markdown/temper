# Session ledger — the assistant's cross-session working state

Maintained by the interactive session assistant; humans welcome; **no
autonomous phase reads or writes this file.** Not intent (that's `specs/`),
not parked ideas (that's `horizons.md`), not work orders (that's the flume
inbox) — the board between sessions. Loaded every session via the `CLAUDE.md`
`@`-import, so every line costs context: target **under ~60 lines**, prune on
graduation, delete over archive.

## Awaiting human ruling

- `(default-assembly-as-data)` fork — recommendation on file: adopt.
  ELEVATED 07-03: now the keystone of the guidance-layer plan — shipped
  opinion packages and requirement-set/scenario assemblies both need the
  floor-as-data mechanism before they can exist.
- **Guidance-layer curation** (07-03 design session): 4 std-lib guidance
  deltas drafted and source-verified (eval-first skill authoring; hooks-over-
  prose now docs-citable; compaction root/nested asymmetry; owner/review-
  like-code) — mapping doc at
  claude.ai/code/artifact/97362c3b-f2eb-4e2a-98de-7a19a29855c8. Boundaries
  settled in conversation: authority = package identity (field practices
  never enter `*.anthropic`); opinion is opt-in, never the floor; templates
  scaffold once, never reconcile back; runtime practice is out of lane.
  Rider: AGENT-KIND's "no story demands it" deferral is challenged by this
  story — kinds are package sockets (agent, hooks, settings, mcp).
- **Surface reformulation v2 — "the scripted altitude"** (07-03 design
  session; John strongly endorsed, NOT yet ratified — needs his cold read):
  members become typed TS modules, composition = imports, `temper emit`
  compiles the library → `.claude/` + inert manifest TOML + lock
  (double-emit determinism); gate stays Rust, no Node. Inverts the ratified
  20-surface authority Decision's *medium* (the mirror was illegible:
  same-medium source/output; .ts→.md reads as src→dist). Flume bridge rides
  behind it: .flume/ as third landscape (pending.json citation-resolution
  checks = the custom-kind story's next demo), prompts join the library,
  `temper check` as a tick gate → the flume fence ratchets open under
  contract. Prototype + full cost ledger:
  claude.ai/code/artifact/3b82d365-492d-4900-ad41-e00feb755a07. Path if it
  survives: hands-on SPIKE (config + two member modules + throwaway emit,
  byte-diff against current .claude/) BEFORE any corpus re-cut. Riders: SDK
  becomes load-bearing product; npm supply chain at emit; template-literal
  prose tax accepted verbally 07-03 — re-verify in the spike. Subsumes the
  pull-model membership idea; reshapes (default-assembly-as-data) and the
  guidance-layer plan (opinion packages/templates become typed imports).
  Final rung (§9-10): spec kind — requirements as exports, satisfies as
  imports (the intent graph IS the module graph); prose as tagged-template
  program over the ontology, mentions = declared one-way edges (law 8's
  mention/edge split dissolves). HARD RIDER: interpolation opt-in per
  mention forever — a mention-completeness check is the mining swamp
  rebuilt (law 4); never file one.
- `format` frontmatterless vocabulary member — parked until a check needs the
  distinction (vocabulary without a consumer is rejected doctrine).

## Held ceremonies (human halves queued behind slices)

- **Drift re-cut**: rides behind the shipped surface-authority lock
  (`20-surface.md` Decision, ratified 2026-07-03); re-cut content vs structure
  drift in 20-surface + the `drift-engine` join's `means` once the lock
  proves the inversion.

## Verify when the loop drains

- **Trailing-period @import** (cascade CLAUDE.md:26, found by the first
  tree-wide vet): our extraction reads the target as `collaboration.md.` —
  unbacked. Whether Claude Code's parser strips trailing punctuation is
  UNVERIFIED — and the live memory docs (fetched 07-03) are silent on it, so
  only an empirical test settles it. If it doesn't strip → cascade has a
  real dead import (report to John, one-char fix his side); if it does →
  slice a punctuation nuance into the at-import grammar (cited). Either way
  it's a wedge story for the public docs.
- Lock artifacts: exercise authority = "surface" end to end once installed
  (guard hook blocks, gate-installed enumerates it).

## Standing discipline (lessons paid for)

- Wake-then-loop as its own background task; never orphan it in a pipe.
- The loop shares this working tree — `git status` before any
  rebase/stash/restore; its mid-tick dirty files are not mine to touch.
- Never `git restore` an uncommitted authored file (lost the 50-distribution
  header once).
- Curated files are compiled-in embeds: parse slice first, curated line after
  (the red-interim trap). Check `build.rs`/walk assumptions before moving or
  nesting curated trees.
- Placement attempts are integration tests: placing colliding curated files
  flushed five single-provider assumptions; expect the pattern to recur.
- `cargo-insta` is not installed — accept snapshots by `mv .snap.new .snap`.
- The session-start gate runs the PATH binary, which goes stale against a
  fast-moving surface (Jul 3: a 2-day-old binary false-blocked a session with
  19 phantom danglings). `cargo install --path .` after engine waves. Product
  gap worth a ruling: the gate can't tell "surface is wrong" from "I am old"
  — a binary-vs-lock freshness check would make that a clean finding instead
  of a false red (false gate-blocks erode the gate — law 1's trust).

## Broad goal: a consolidated v0.1 release + open-source publish

Set 2026-07-03. Steer sessions toward shipping, not just deepening:

- **README stands alone**: the operational story (what it does, run it, read
  the findings) lives IN the README — no reader sent into `specs/` to
  understand the product; specs stay the internal contract.
- **Public-facing docs**: a plain-language docs set — the jargon in this repo
  (joins, worlds, arities, flattening) is internal vocabulary; the public
  docs speak user (broken imports, dead rules, drift, one gate).
- **Release mechanics**: PACKAGING-CHANNELS (needs John's credentials),
  COMMUNITY-DOCS fence widen, prebuilt binaries per 55-offering's launch
  Decision, version/tag discipline.
- Weigh new machinery against this goal — depth that v0.1 doesn't need can
  wait (the horizons file exists for a reason).

## Standing offers on the human board

- `(code-seam-joins)`: unblocked, wants its tag-grammar design session.
