# Plan state

- **Phase:** the contract engine is feature-complete for the decidable in-crate
  algebra across the **skill** and **rule** kinds, and self-host is green. The
  frontier is the spec landscape (declared model + dependency graph,
  `specs/30-landscapes.md`), gated on the `(model-declaration-format)` human fork.
- **Last shipped:** RULE-CHECK (6a1a5d8 / 82db54c). Verified on disk: `check`
  dispatches each artifact to the contract for *its* kind and merges the
  diagnostics (`main.rs:86-96`), embedding `contracts/rule.toml`
  (`main.rs:36`); the self-host proof is green — `temper` lints its own
  `.claude/rules/` clean (`tests/cli.rs:218`).
- **In flight:** nothing committed; working tree clean. The engine stubs
  `require_sections` to `Outcome::Indeterminate` (`engine.rs:191`) because
  `Features` carries `body_lines` but no headings (`extract.rs:59`) — a declared
  clause that silently neither passes nor fails.
- **Next:** REQUIRE-SECTIONS is pickable (`open`, fork-free) — extract body
  headings and decide the clause, closing that gap. The two larger in-scope items
  (declared model, dependency graph) and `dependency-exists` stay blocked on
  `(model-declaration-format)`; the full `pattern` primitive on `(regex-crate)`.

Plan continues: no — pending was empty and the stale `state.md` is reconciled
(RULE-CHECK shipped); one pickable `open` entry is filed, the inbox is empty, and
no fork moved. Build drains from here.
