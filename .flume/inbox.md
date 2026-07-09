<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->

- Decision 0003's own consequence, half-shipped and stranded — filed as real
  work, not "someday." 0003 (`specs/decisions/0003-selectors-are-atomic.md`)
  rules `requirement.kind` "recuts from a selection input into a shipped
  clause" — narrowing a requirement's satisfier set by kind must be an
  each-grain clause, never a second selector, else a wrong-kind opt-in is a
  silent exclusion instead of a finding. `SATISFIER-KIND-CLAUSE` (94ac5f1,
  shipped 2746b11) delivered the Rust-engine half: `builtin::
  kind_narrowing_clause(kind: &str)` synthesizes a live `Predicate::Kind`
  clause at check time off `RequirementRow.kind: Option<String>` — confirmed
  live in `src/roster.rs`/`contract.rs`/`engine.rs` today. Its own commit
  body scopes this deliberately: "never stored as a lock `ClauseRow`...
  `builtin.rs`/`drift.rs`/`sdk/` carry no new row shape" — reasonable for
  that one entry, but the SDK-side half 0003 names directly never followed.
  `sdk/src/contract.ts`'s `Requirement.kind?: KindDefinition<object>` is
  untouched since `9dc9162` (2026-07-04) — three days *before* 0003 was
  ratified (`git log -S"kind?: KindDefinition" -- sdk/src/contract.ts`
  shows exactly one commit, ever). Concretely broken today: `kind: skill`,
  `kind: agent`, `kind: command` all fail to type-check (TS2322 — each
  kind's required `description` field breaks the callable's contravariance
  against `KindDefinition<object>`); only `kind: rule`/`kind: memory` work,
  since those two kinds carry no required field. The field's declared TYPE
  was never recut to match what the engine has read since Jul 7 — a bare
  kind-name string (`requirement.kind?.key` is the sole read site,
  `sdk/src/declarations.ts:326`; nothing ever calls `.kind()` as a
  constructor). `SATISFIER-KIND-CLAUSE`'s own tests exercised `kind: rule`
  only, so the 3-of-5-kinds authoring gap shipped invisibly and no
  subsequent residue sweep named it (nothing routes an unshipped
  decision-consequence by symbol grep the way retired vocabulary does).
  Fix shape: retype `Requirement.kind` to a narrower structural interface —
  `{ readonly key: string }`-ish, satisfied by every `KindDefinition<T>`
  regardless of `T`'s required fields since only `.key` is ever read — never
  a plain string, preserving "identity travels by import" (`specs/model/
  contract.md`, "requirement"). No Rust-side change; no lock/SEAM_VERSION
  bump; `sdk/test/refusals.test.ts`'s existing `kind: rule` cases plus a new
  `kind: skill`/`kind: agent`/`kind: command` case should cover it. Observed
  at 0f5fcd0.

- The embedded-kind `render` hook bypasses leaf mention resolution — routed
  from friction capture `build-embedded-leaf-text-render-hook-gap` (drained).
  EMBEDDED-LEAF-TEXT (18d3406) made the default view (`emit.ts`'s
  `renderMemberToml` leaf loop) resolve a `Text` leaf's mentions — loud on a
  dangling address — before quoting; EMBEDDED-KIND-RENDER-HOOK (3c6f50b) hands
  a kind's own `render(value)` the *raw* `EmbeddedMemberValue`, leaves
  unresolved. Net: the identical dangling leaf mention refuses loudly on a
  hook-less kind and silently stringifies (`[object Object]`-shaped) on a kind
  that declares `render` — refusal depends on an unrelated authoring choice.
  That contradicts `contract.md` ("edge"): every edge resolves into one
  enumeration shared by the gate and every read verb. (The capture cited a
  "one display rule" framing that does not exist in the corpus — this
  enumeration clause is the actual ground.) Fix shape: resolve every leaf
  (mention-checked) before invoking `render`, so a hook always sees plain
  strings and the refusal bar is uniform. Rejected alternative: document that
  hooks own their own resolution and re-export `resolveLeaf` through
  `index.ts` — preserves raw-template access nothing yet wants, at the price
  of a per-hook refusal bar. Test ask: `render` + `Text` leaf + dangling
  mention refuses identically with and without the hook. Seam:
  `sdk/src/emit.ts`, `sdk/src/kind.ts` (hook signature), `sdk/src/prose.ts`
  (`resolveLeaf`). No Rust-side change expected. Observed at a036328.
