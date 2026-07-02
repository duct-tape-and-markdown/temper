+++
# The spec kind's package — temper's own, project-authored (the dogfood).

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 150
guidance = "The ~150-line budget from specs/90-spec-system.md: a long spec is a smell — split the concern — never a build-breaker. The corpus deliberately carries violations (20-surface, 50-distribution) as a live test that the advisory tier informs without blocking."
source = "specs/90-spec-system.md (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "section_contains"
heading = "Decision:"
marker = "Rejected"
guidance = "Every Decision block names its rejected alternative — a decision without the road not taken is a preference, not a decision (specs/90-spec-system.md, Decisions). The colon prefix scopes to real blocks (`Decision: <title>`), not sections about decisions."
source = "specs/90-spec-system.md (retrieved 2026-07-02)"
+++

# spec

The require-side of temper's own `spec` kind. Two further clauses from the
worked example (`specs/15-kinds.md`) are deliberately absent until their
machinery ships: decisions-name-alternatives (the `section_contains` predicate,
enumerated in `specs/10-contracts.md` but not yet in the engine) and
references-resolve (waits on declared-normalization edges). Declaring a check
the engine cannot decide would fake a gate — law 3.
