# plugin-manifest types outputStyles, workflows, experimental

Shipped as named. Re-fetched plugins-reference 2026-11-26:

- `#unrecognized-fields` still exists, but it covers unknown keys, not type
  mismatches. The type-mismatch passage sits under `#validate-the-manifest`
  ("Validation failed: the manifest has a type mismatch ... Claude Code
  reports the same problem when it loads the plugin"). The page no longer
  says a wrong-typed field stops the plugin loading, so the `keywords`
  guidance claimed more than its source. It now says a mismatch fails plain
  validate. The closed-keys cite keeps `#unrecognized-fields`, which is
  correct for it.
- `examples/base-harness` adopts no plugin-manifest contract, so its lock
  did not move.
- `manifest_schema_oracle` EXPECTED_LAG drops `outputStyles`. The vendored
  schema has no `workflows`/`experimental`, so neither shows in the oracle.

Open, not taken: `experimental`'s inner keys (`themes` path|array,
`monitors` path|inline array, `evals` path|array) stay untyped. A nested
`type` needs a dotted-field address the vocabulary may not spell. That is
a possible hold to key if plan wants it.
