<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- 2026-07-02 (human): WEDGE-INSTALL-SUMMARY — on an unconfigured harness the
  zero-config wedge's output is one install.gate-installed advisory PER
  artifact (20 on the first real vet target), drowning the actual health
  signal. Collapse the install story to ONE summary advisory ("temper's gate
  is not installed — run `temper install` (session hook, CI job, N modelines
  missing)") and let artifact findings own the output. The per-artifact detail
  stays available behind the summary (a count/list), not as 20 siblings.

- 2026-07-02 (human): WEDGE-COVERAGE-NOTE — the wedge is silent about surfaces
  it has no kind for (a real vet target carried an agents/ dir, hooks,
  settings.json, CLAUDE.md — pre-memory-kinds — and a specs corpus; the output
  said nothing). Silence reads as "checked"; that violates the fail-loud
  invariant (specs/architecture/50-distribution.md). Emit a coverage note:
  which kinds checked N members, plus known-but-unmodeled harness surfaces
  present on disk (from the loci the built-in set knows about vs what exists).
  Scope honestly: only name surfaces temper can cheaply detect; no guessing.

- 2026-07-02 (human): SKILL-VERSION-EXTRACTION-DROP — the curated skill kind
  extracts a `version` field (kinds/claude-code/skill/KIND.md) documented in
  NEITHER the agentskills.io spec nor Claude Code's frontmatter table (both
  re-fetched 2026-07-02, wave-1 grounding), and skill.anthropic's own prose
  says "`metadata` is the sanctioned home for versioning — there is no
  top-level `version` field." The extraction is an uncited leftover. Slice:
  re-pin every test/fixture leaning on `version` as the example declared field
  (src/import.rs:914, src/frontmatter.rs:606-612, src/engine.rs:529,
  tests/apply.rs:222, tests/readd.rs:202, extract_equivalence snapshot) onto
  `license` (which IS in the standard), so the curated line can be dropped by
  the human without reddening the tree. The KIND.md edit itself stays human
  (kinds/ is fenced).
