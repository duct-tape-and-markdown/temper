+++
# The process class's package — temper's own, project-authored (the dogfood).

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 150
guidance = "The ~150-line budget from specs/process/90-spec-system.md: a long spec is a smell — split the concern — never a build-breaker. The corpus deliberately carries violations (90-spec-system itself rides the line) as a live test that the advisory tier informs without blocking."
source = "specs/process/90-spec-system.md (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "section_contains"
heading = "Decision:"
marker = "Rejected"
guidance = "Every Decision block names its rejected alternative — a decision without the road not taken is a preference, not a decision (specs/process/90-spec-system.md, Decisions). The colon prefix scopes to real blocks (`Decision: <title>`), not sections about decisions."
source = "specs/process/90-spec-system.md (retrieved 2026-07-02)"
+++

# process

The require-side of the `process` class (`specs/process/90-spec-system.md`,
"Decision: classes are kinds, discriminated by placement" — each class binds
its own package). Process members publish no entities and satisfy none — the
demand/claim pairing is the intent/architecture seam — so this package is the
corpus-wide structure clauses and nothing else, and stays that way.
