# Plan state

- **Phase:** the **rule** artifact kind — second instance of the engine, toward
  self-hosting (`specs/20-surface.md`, "Artifact kinds & contract selection").
- **Last shipped:** the contract-engine cutover (8ce0842 — `check` runs
  `engine::validate` against the embedded skill template; the heuristic registry,
  `src/rules.rs`/`tests/rules.rs` and its fixtures, are gone), then the rule-kind
  spec + curated `contracts/rule.toml` (a6497ec).
- **In flight:** nothing committed; working tree clean. Verified on disk:
  `src/rules.rs` absent, `lib.rs` has no `rules` module, `main.rs` validates via
  the generic engine, `acceptance.rs` drives the contract path. `contracts/rule.toml`
  loads but nothing embeds/consumes it yet; `import`/`Workspace`/`check` know only
  the skill kind. Self-host fixture is real: `rust.md` has `paths:`, `collaboration.md`
  has no frontmatter.
- **Next:** RULE-IR (open) and RULE-CONTRACT-TEST (open) are pickable now →
  RULE-IMPORT (blockedBy RULE-IR) → RULE-CHECK (blockedBy RULE-IMPORT), which lands
  the self-host check on temper's own `.claude/`.

Plan continues: no — both shipped cutover entries verified gone and dropped, the
inbox is drained (RECONCILE done; NEW SLICE filed as the four RULE-* entries),
and two pickable `open` entries lead the chain. Building drains the queue from here.
