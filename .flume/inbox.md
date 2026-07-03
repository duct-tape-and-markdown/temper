<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): WAVE 1, memory kinds — five slices, dependency-ordered.
  Context: the first foreign-provider kinds (agents-md.memory, claude-code.memory
  — deliberately colliding on bare `memory`) are drafted and cite-verified; the
  four curated files are held by the human and CANNOT be committed until slice 1
  ships (co-embedding two `memory` carriers turns cargo test red via eager
  resolution). Slices below carry file:line evidence; order matters.

- 2026-07-02 (human): MEMORY-COLLISION-SCOPE (slice 1, blocks the wave):
  `builtin_kind::definitions()` and `resolve()` (src/builtin_kind.rs:85-96,
  :112-118) call `CustomKind::resolve_bare` for EVERY embedded kind against the
  whole embedded set and propagate the first AmbiguousKind via `?` — so two
  co-embedded `memory` kinds fail definitions() for every caller, whether or not
  anything references bare `memory`. That is stricter than the Decision's
  "nobody pays a qualification tax until two providers actually meet"
  (specs/architecture/15-kinds.md): a collision should cost only the caller who
  binds/references the ambiguous BARE name; qualified lookups and non-colliding
  bare names stay clean. After it drains, the human commits the four curated
  files (two KIND.md + two PACKAGE.md) together.

- 2026-07-02 (human): IMPORT-BUILTIN-SCAN-GENERIC (slice 2, independent):
  import::run()'s built-in scan is hardcoded to the two literals in
  `crate::kind::BUILTIN_KINDS = ["skill","rule"]` (src/kind.rs:30) with two
  call sites `import_frontmatter_kind(..., "skill"/"rule")`
  (src/import.rs:123-124). Drive the scan off `builtin_kind::definitions()`
  generically — the doc comment already claims "one generic adapter … no
  per-kind writer" (src/import.rs:120-122). Until it lands the memory kinds
  are embedded but discover nothing (their prose says so honestly).

- 2026-07-02 (human): CHECK-WORKSPACE-KIND-MAP (slice 3, after slice 2):
  `check::Workspace` hardcodes `skills: Vec<Member>` / `rules: Vec<Member>`
  (src/check.rs:34-38); a third built-in kind's members would be
  extraction-checked but never clause/package-checked. Replace with a generic
  per-kind member map so memory.anthropic's clauses actually run against an
  imported CLAUDE.md member.

- 2026-07-02 (human): DECLARED-FRONTMATTER-ADAPTER-CUSTOM (slice 4,
  independent, correctness/generality not a memory blocker):
  `import_custom_unit` (src/import.rs:362-409) ignores `unit_shape`/`format`,
  deriving unit id from file_stem and treating the whole file as opaque body —
  accidentally correct for a frontmatterless File-shaped kind, wrong in
  general. Make the declared unit_shape/format load-bearing on the custom
  path (the seam is already named at src/kind.rs:77).

- 2026-07-02 (human): RECURSIVE-GOVERNS + PLACEMENT-QUALIFIED-ID (slice 5,
  independent, largest; the two halves land together): (a) an any-depth
  `governs.glob` mode — `collect_glob`/`glob_matches` (src/import.rs:295-349,
  :453-484) express fixed-depth single-`*` segments only; (b) fold directory
  placement into file-shaped unit identity — both id paths
  (src/frontmatter.rs:186-196; src/import.rs:376-381) collapse same-named
  nested files to their stem, so nested AGENTS.md/CLAUDE.md would clobber each
  other's surface entry. Needed to model the DEFINING nested nearest-wins
  hierarchy of both memory formats (agents.md documents nesting; OpenAI's main
  repo ships 88 of them). Recursion without placement-folded ids is useless —
  one slice, two halves.
