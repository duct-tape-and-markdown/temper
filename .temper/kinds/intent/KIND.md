+++
governs = { root = "specs/intent", glob = "*.md" }

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "placement"

[[extraction]]
primitive = "sections"

[[extraction]]
primitive = "fenced"

[[genres]]
name = "decision"
leaves = ["chosen"]
collections = ["rejected"]

[[genres]]
name = "law"
leaves = ["statement"]
collections = ["bounds"]

[[genres]]
name = "bound"
leaves = ["claim", "deferred", "unlock"]
collections = []
+++

# The intent kind

An intent spec is the why and the law — temper's north star, model, and
offering, evergreen and human-authored (`specs/process/90-spec-system.md`,
"the corpus is classed"). Placement is the class: living under `specs/intent/`
is the authored act that makes a file intent, never a filename or shape
convention (same doc, Decision). The extraction surfaces the line budget and
the heading/section structure — markdown structure only, no body-mined
references (`specs/architecture/15-kinds.md`; law 8): backtick file mentions
in spec prose are typography, never edges.

The class's distinguishing edge is the demand side of the corpus graph: an
intent member declares the entities it defines in its surface header, each a
member-published requirement that the concept be given an architecture home.
That header authoring is the migration's next stage — until it lands, this
kind carries the placement and structure only.
