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
- **Scripted altitude: RATIFIED; floor wave SHIPPED 7/7** (07-03; `specs:`
  32ea84d; pre-state = `mirror-era` tag; wave = READD-RETIRE →
  EMIT-OWNED-PLACEMENTS, drained + pushed; two self-gate reverts, both
  self-healed via inbox-diagnosed legacy-read fallbacks). Dogfood
  regenerated (`chore(harness)` ae0626c: 17 `[[member]]` tables, lock on
  source_hash/emit_hash; gate green on the manifest path); install+emit
  seam verified end to end. REMAINING ON JOHN: ask (a) — scaffold the
  SDK/npm authoring face (the altitude rung waits on it). Residual:
  prose-tax re-verify in the first altitude slice; corpus module migration
  is a staged ceremony. Design record:
  claude.ai/code/artifact/3b82d365-492d-4900-ad41-e00feb755a07.
- **Authority territories, not "human" territory** (07-03, John's ruling in
  session): the fence protects the authority moment + reviewability, never
  authorship — nearly every byte is agent-drafted. chain.ts re-cut landed
  (3f431fe): machine outputs (temper.toml, lock) build-writable
  regenerate-only; fenced territories renamed RATIFICATION territory.
  Doctrine's spec half rides the addressable-corpus ceremony: intent is
  human-*ratified*; autonomous drafting rights widen per document with the
  genre gradient, propose-only; authority never loosens.
- **The addressable corpus: RATIFIED + floor slice SHIPPED** (07-03;
  `specs:` 52f149c Δ1–Δ6; pre-state = `bound-prose-era` tag; cold read
  confirmed). Engine halves shipped by the loop in 9 ticks, zero reverts
  (FENCED-PRIMITIVE → GENRE-MANIFEST-LEAF → IMPACT-LEAF-GRAIN →
  CONTEXT-VERB, + INSTALL-DRIFT-STRINGS); genre package landed in-session
  (5f56fda: decision/law/bound on the spec kinds); verbs exercised, the
  mixed-rung disclosure live ("13 documents below rung 3"). D1–D7 record:
  claude.ai/code/artifact/8894d9ee-a143-422b-84b3-07f7140e248c; deltas as
  ratified: claude.ai/code/artifact/df62b0a2-dcc7-4e11-a63a-8fc0b7798127.
  NEXT: rung-3 pilot (45-governance or 15-kinds) is BLOCKED on the display
  rule (inboxed — emit has no custom-kind projection face; a fence would
  project as raw TOML). Protocol stands: per-document, residual =
  connective tissue only, don't freeze the package until 3–4 documents
  through. Altitude authoring still on ask (a).
- `(default-assembly-as-data)` addendum: subsumed-in-shape by the manifest
  model (a shipped default assembly = embedded manifest data) — formally
  close it or let plan carry the reconcile.
- `format` frontmatterless vocabulary member — parked until a check needs the
  distinction (vocabulary without a consumer is rejected doctrine).

## Held ceremonies (human halves queued behind slices)

- **Drift re-cut**: the wave shipped the lock inversion — now unblocked;
  re-cut content vs structure drift in 20-surface + the `drift-engine`
  join's `means`. Could ride the addressable-corpus ceremony.

## Verify queue

- **Trailing-period @import** (cascade CLAUDE.md:26): extraction reads the
  target as `collaboration.md.` — unbacked. Whether CC's parser strips
  trailing punctuation is UNVERIFIED (live docs silent, 07-03); only an
  empirical test settles it. Doesn't strip → real dead import (one-char
  fix, John's side); strips → cited nuance into the at-import grammar.
  Wedge story for the public docs either way.
- Guard hook is advisory-only (echoes to stderr, always exit 0) — verify
  that matches 50-distribution's intent for the write guard, or it's a
  finding.

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
