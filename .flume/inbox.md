<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.
-->

- RECONCILE: `CHECK-CUTOVER` and `RETIRE-HEURISTICS` are SHIPPED (hand-landed,
  commit 8ce0842) — verify on disk (`src/rules.rs` gone, `src/main.rs` runs
  `engine::validate`) and DROP both from pending.

- NEW SLICE — the **rule** artifact kind (toward self-hosting; `specs/20-surface.md`
  "Artifact kinds & contract selection", `(contract-selection)` now RESOLVED).
  Derive entries for: (1) a rule extractor — parse `.claude/rules/*.md` into the
  same `Features` view the engine validates (frontmatter `paths` optional + a
  byte-faithful body); model it as a `rule` artifact kind in the IR/`Workspace`.
  (2) `import` also scans `<harness>/.claude/rules/*.md` (today it scans only
  `skills/*/SKILL.md`). (3) `check` dispatches each artifact to the built-in
  contract for its kind — embed `contracts/rule.toml` (human-authored, done)
  alongside the skill contract; skill→skill contract, rule→rule contract.
  (4) tests, incl. a `contracts/rule.toml` load/clause-vector test (mirror
  `tests/contract_template.rs`). Scope each entry's `files` to the truthful blast
  radius (existing tests that change). contracts/ is human territory — build
  EMBEDS rule.toml, never writes it. Acceptance for the slice: `temper import`
  picks up rules, and `temper check` on a harness with rules validates them
  against the rule contract — including `temper`'s OWN `.claude/` (self-host).
