# Rule size advisory: shipped, with an engine widening the entry did not name

- **Engine widening.** `extent` was refused in any `when` body (`bodyless` at
  `Binding::Element`), so "guard on `paths` presence" could not be written.
  Derived from `specs/model/contract.md` ("Guard and body share one address
  binding"): a guard over a top-level field (no `[*]`) locates the member
  itself. So its body now binds as `Binding::Member` at admissibility and is
  judged over the member's own features (`src/engine.rs`). Each-grain guards
  keep the element fence. The fence pin's `when_over` now uses an each-grain
  guard (`agents[*].model`). Plan may want this stated in the spec section.
- **Guard spelling.** `type("paths", ["string", "list"])`: a value of either
  documented shape opens the guard. The vocabulary has no bare presence guard.
- **Docs re-verified 2026-11-26** (memory, "Claude ignores the end of my
  subdirectory CLAUDE.md"): 40,000 chars at 200K; at 1M, 200,000 on
  Opus/Sonnet 4.6 and 150,000 on later models. Applies since v2.1.309. The
  clause judges 40,000 and the guidance says so.
- **Adjacent fix.** The 200-line `rule.extent` guidance said "rules load in
  full regardless of length", which the docs now contradict for on-demand
  rules. It is corrected in place.
- **Uncovered, same doc section:** subdirectory `CLAUDE.md`/`CLAUDE.local.md`
  and the files they `@`-import are cut at the same limit. So are rules in a
  *subdirectory's* `.claude/rules/`. No memory-kind clause judges them yet.
  The `rule` locus is root `.claude/rules/**`.
