# Session ledger — the assistant's cross-session working state

Maintained by the interactive session assistant; humans welcome; **no
autonomous phase reads or writes this file.** Not intent (that's `specs/`),
not parked ideas (that's `horizons.md`), not work orders (that's the flume
inbox) — the board between sessions. Loaded every session via the `CLAUDE.md`
`@`-import, so every line costs context: target **under ~60 lines**, prune on
graduation, delete over archive.

## Awaiting human ruling

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
- **Scripted altitude: RATIFIED, corpus re-cut LANDED** (07-03, cold read
  held; `specs:` commit 32ea84d; pre-state = `mirror-era` tag). The corpus
  now carries it — authoring face / manifest / carriage / mentions /
  gradient / init/emit; re-add retired; never-climb is law 8 text; .flume/
  named candidate third landscape. Plan reconciled (9e71a1e): filed
  `(scripted-altitude-reconcile)` as the frontier fork — corpus decided,
  code not migrated; floor deltas entangled (one serialized wave), altitude
  needs SDK scaffolding. TWO ASKS ON JOHN: (a) scaffold the SDK/npm
  authoring-face surface; (b) rule the floor wave's sequence (or delegate
  the serialization to plan) — then the blockedBy chain files. Also
  residual: prose-tax re-verify in the first altitude slice; the corpus's
  own module migration is a staged ceremony. Design record:
  claude.ai/code/artifact/3b82d365-492d-4900-ad41-e00feb755a07.
- `(default-assembly-as-data)` addendum: subsumed-in-shape by the manifest
  model (a shipped default assembly = embedded manifest data) — formally
  close it or let plan carry the reconcile.
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
- **Two projectors, one file** (07-03, hit during the re-cut): `apply`'s
  re-emission of a rule drops `install`'s schema modeline + reflows the
  YAML — hand-reverted this session. Under the re-cut spec emit owns the
  projection whole, so install's placements must ride emit or declare their
  lines as emit-owned. Needs an entry; don't run bare `apply` on rules
  until it lands.

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
