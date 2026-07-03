+++
# The architecture class's package — temper's own, project-authored (the dogfood).

[[clause]]
severity = "advisory"
predicate = "max_lines"
max = 150
guidance = "The ~150-line budget from specs/process/90-spec-system.md: a long spec is a smell — split the concern — never a build-breaker. The corpus deliberately carries violations (20-surface, 50-distribution) as a live test that the advisory tier informs without blocking."
source = "specs/process/90-spec-system.md (retrieved 2026-07-01)"

[[clause]]
severity = "required"
predicate = "section_contains"
heading = "Decision:"
marker = "Rejected"
guidance = "Every Decision block names its rejected alternative — a decision without the road not taken is a preference, not a decision (specs/process/90-spec-system.md, Decisions). The colon prefix scopes to real blocks (`Decision: <title>`), not sections about decisions."
source = "specs/process/90-spec-system.md (retrieved 2026-07-02)"
+++

# architecture

The require-side of the `architecture` class (`specs/process/90-spec-system.md`,
"Decision: classes are kinds, discriminated by placement" — each class binds
its own package). Today it carries the corpus-wide structure clauses; the
class's own clause — `satisfies` claims naming intent-declared entities, the
opt-in fill edge — lands with the surface-header authoring, the migration's
next stage. Declaring a check before its declarations exist would fake a gate
— law 3. There is no references-resolve clause and never will be: body-mined
references are retired (`specs/architecture/15-kinds.md`, "no body-mined
references").
