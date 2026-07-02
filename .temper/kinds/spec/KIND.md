+++
governs = { root = "specs", glob = "*.md" }

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "placement"

[[extraction]]
primitive = "sections"
+++

# The spec kind

A spec is temper's own governing document — one file per concern under
`specs/`, evergreen, human-authored (`specs/90-spec-system.md`). The extraction
surfaces the line budget and the heading/section structure — markdown structure
only, no body-mined references (`specs/15-kinds.md`, "no body-mined
references"; law 8): backtick file mentions in spec prose are typography, never
edges.

The corpus's real edges are declared, not extracted: the classed-corpus design
(`specs/90-spec-system.md`, "the corpus is classed") pairs intent-declared
entities with architecture `satisfies` claims in member headers. This single
`spec` kind is transitional until that migration splits it into the three class
kinds.
