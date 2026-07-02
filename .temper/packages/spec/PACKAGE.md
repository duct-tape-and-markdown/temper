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

The require-side of temper's own `spec` kind. The class pairing from the worked
example (`specs/15-kinds.md`) — an intent member's declared entities satisfied
by an architecture member — is deliberately absent until member-published
requirements ship and the corpus migrates to its three class kinds
(`specs/90-spec-system.md`, "the corpus is classed"). Declaring a check the
engine cannot decide would fake a gate — law 3. There is no references-resolve
clause and never will be: body-mined references are retired
(`specs/15-kinds.md`, "no body-mined references").
