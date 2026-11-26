# NARRATION-MATCHES-THE-MODEL

Shipped; net -48 lines.

- The "temper on the PATH" default sites in contract.ts/builtins.ts were already gone
  when this tick started (the tool lowering had restated them as "the temper package's
  bin"). Nothing left to do there.
- The census also found a false shipped guidance string: the mention-reachable
  default clause said containment is "literal" (a glob must appear verbatim in the
  gate). graph.rs `uncontained` actually samples paths from each scope glob. The
  guidance is restated in builtins.ts, builtin_lock.toml and bundled_reference, along
  with the contract.rs/contract.ts/graph.rs docs and SDK_COUNSEL. graph.rs's own doc
  had said the sampling "errs toward firing", but a sample errs toward silence. The
  docs now say only that a verdict can be wrong.
- SDK_COUNSEL now takes the rule names from COUNSEL_RULE_NAME and
  GUIDANCE_RULE_PREFIX, which moved into counsel.ts. bundle.rs hook_events and
  hooks.json share one const.
- Twenty-odd comments cited build-entry tags such as `(READ-EDGE-UNIFY)` as
  history. The tags are stripped.
- Dogfood: SDK_COUNSEL moved, so `.claude/rules/temper-sdk.md` and `.temper/lock.toml`
  drift until a human re-emits. The example harness was re-emitted here.
- Left (other families, not this entry's): install.rs still spells "SessionStart"
  by hand about eight times beside GATE_HOOKS. `cargo doc` has existing
  private-intra-doc-link warnings in admissibility.rs.
