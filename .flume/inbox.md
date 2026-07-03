<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-03 (human): WEDGE-FACT-FLOOR — human ruling: first contact is the
  hook, so the zero-config wedge validates everything fact-shaped. Run
  directive-target classing (graph.directive-unbacked) on the FLOOR tier —
  no assembly required — as a non-gating advisory; an assembly keeps its
  existing power to escalate. Rationale: an unbacked import is a pure fact
  (the file is not there), not a graph-scope opinion; the reachability
  severity ruling (assembly-declared) is about gating, not visibility.
  Repro of today's gap: harness with CLAUDE.md `@docs/missing.md` and NO
  temper.toml → currently silence; expect the advisory.

- 2026-07-03 (human): WALK-IGNORE-DISCIPLINE (blocks the memory tree flip):
  discovery walks have no ignore rules — `collect_glob` descends everything
  and `repo_file_set` (src/main.rs:893) even collects `.git/` contents — so
  flipping the memory kinds' governs to `**/CLAUDE.md`/`**/AGENTS.md` would
  import strangers' files out of node_modules as members. Slice: DISCOVERY
  respects ignore rules (.gitignore + always-exclude .git) — a member is
  authored content. The directive-BACKING file set stays raw disk: the
  harness loads an @import target regardless of gitignore, so pruning the
  backing set would forge unbacked findings (law 3) — two different sets,
  keep them distinct. After it drains, the human flips both memory kinds'
  governs to the any-depth glob (curated embeds, human territory).

- 2026-07-03 (human): IMPACT-VERB — ratified and graduated from horizons
  (specs/architecture/20-surface.md, "CLI surface": `temper impact <member>`).
  Slice: the read verb — deterministic tier-1 traversal over the composed
  graph (joins, activation, directive edges): what requirements go unfilled,
  what satisfies dangle, what directive edges lose their backing, whose
  reachability dies, if <member> is removed or renamed. Read-only, narrated
  like why/requirements. CI-comment placement is a later slice, not this one.

- 2026-07-03 (human): AUTHORITY-POSTURE-PARSE (first lock slice; ratified
  Decision at specs/architecture/20-surface.md, "surface authority is a
  declared posture"): the assembly accepts `authority = "shared" | "surface"`
  (top-level key, default shared, unknown value = load error like every
  closed vocabulary). Inert this slice — parse + expose on the composed
  layer + admissibility test. The red-interim pattern: parse first,
  consumers follow.

- 2026-07-03 (human): INSTALL-GUARD-ARTIFACTS (after AUTHORITY-POSTURE-PARSE):
  `install` wires the enforcement artifacts per the posture — the managed-by
  note on projections whose format tolerates cost-free metadata (skip memory
  projections: a comment in CLAUDE.md costs context every session; the note
  rides the modeline machinery, NEVER apply — law 5), and the PreToolUse
  guard hook (warn-and-route under shared, block under surface), enumerated
  in gate-installed's audit like the existing hook/CI/modelines. The
  Decision's stated limit goes in the hook's message verbatim honesty: other
  tools' writes are not bound by it.
