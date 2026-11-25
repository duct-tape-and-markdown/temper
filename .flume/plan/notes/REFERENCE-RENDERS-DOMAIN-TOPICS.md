# Reference renders domain topics as supporting-doc children

- `sdk/src/reference.ts`: `withReference` now puts each rendered reference
  next to one `supporting-doc` member per committed requirement, named
  `domain-<r>` and hosted by the rendered skill. It lands at
  `.claude/skills/temper/domain-<r>.md`. The body is a `# The <r> domain`
  heading, then the prose, `## Verifier`, and `## Owners`. Under Verifier it
  says "It is held by review: <prose>", by script, by telemetry, or "It is
  unheld — no verifier is declared ...". Owners are satisfier addresses, or
  "No member fills it yet."
- `ContractedRequirement` (declarations.ts) now carries `verifier`. The
  guidance rules ignore it.
- Side effect: once a reference has any domain, `supporting-doc` is a kind
  in play. Because `withGuidanceRules` runs after `withReference`, a guidance
  rule for `supporting-doc` gets synthesized. The reference's own kind index
  is rendered from the authored program, so it does not list
  `supporting-doc`. The KIND-topics entry should decide whether it renders
  from the pre-topic view as well. If it does, `kind-supporting-doc.md` is
  never linked, but its guidance rule still exists.
- The `kind-<k>.md` links still dangle until the kind-topic entry ships.
- The dogfood `.temper` does not declare `reference()`, so no projection here
  changes.
