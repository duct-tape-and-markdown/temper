<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- `(seam-rows-public-face)` is ruled: **retract the ten seam root exports
  to nothing** — `sdk/src/index.ts`'s `Declarations`/row types/
  `EdgePlacements` plus `SEAM_VERSION`/`compileDeclarations`, and
  `PayloadMember` off `./emit.js`; no subpath home. Human-ruled 2026-07-16
  after a Chesterton check: the exports were born as two-writer scaffolding
  in 9dc9162 (whose own body already called the pipe "internal"), the
  condition retired when 911cc45 made the engine the sole compiler, and
  b5b6fb4 already retracted `declarationsToJson` from the block without
  ceremony. Measured consumers today: zero — `.temper/` imports only
  `emit`/`harness`, the engine mirrors its own `SEAM_VERSION`
  (`src/drift.rs:512`), and `sdk/test/contract.test.ts:25`'s type import is
  a one-line relative retarget to `../src/declarations.js`. The
  SDK-ROOT-EXPORT-CLOSURE datum (6605bf5: the closure pulled
  `EdgePlacements` onto the root because root-exported `compileDeclarations`
  demands its parameter) is evidence *for* retraction — the face grew
  without anyone deciding, and retracting the function removes the pull.
  Rejected on the record: completing the closure over the seam (promotes
  ~28 generated row types to public API, making a Rust column rename a
  consumer break — what "versioned in lockstep" exists to prevent) and a
  subpath home (a quieter public door; `pipeline.md`'s admission condition
  — a consumer exists — is unmet). Accepted residue: `emit()`'s return
  keeps the rows inference-reachable; the boundary is the never-made
  stability promise, not nameability. No spec delta, no decision number —
  `pipeline.md` already rules the payload internal; the code is the
  anomaly. SDK-ROOT-EXPORT-CLOSURE shipped excluding the row family out
  loud, so this is a new entry: retract the exports and re-point the
  closure test's exclusion at the ruled boundary. The fork record deletes
  with the resolving commit. Observed at 2b6bd57.
