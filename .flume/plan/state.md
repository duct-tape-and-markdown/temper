# Plan state

- **Phase:** the **rule** artifact kind — second instance of the engine, toward
  self-hosting (`specs/20-surface.md`, "Artifact kinds & contract selection").
- **Last shipped:** RULE-IMPORT (208e2cc / 7751c16). Verified on disk: `import`
  discovers `.claude/rules/*.md` and writes `<into>/rules/<name>/{meta.toml,
  RULE.md}` + `[[rule]]` roll-up rows (import.rs:101-114, 214-233); `Workspace`
  now carries `rules` loaded name-sorted (check.rs:39, 57-60); `extract::
  rule_features` projects them (extract.rs:136).
- **In flight:** nothing committed; working tree clean. `check` still validates
  *only* the skill contract — `main.rs` projects `ws.skills` against
  `BUILTIN_SKILL_CONTRACT` and never touches `ws.rules` (main.rs:77-94).
- **Next:** RULE-CHECK is now pickable (`open` — its RULE-IMPORT gate landed):
  embed `contracts/rule.toml`, validate `ws.rules` against it, merge with the
  skill diagnostics, and land the self-host proof (`temper check` green over
  temper's own `.claude/rules/` — rust.md/collaboration.md are both clean).

Plan continues: no — RULE-IMPORT shipped, the lone reconciliation was flipping
RULE-CHECK to `open`, inbox is empty, no fork moved, and a pickable `open` entry
leads the chain. Build drains from here.
