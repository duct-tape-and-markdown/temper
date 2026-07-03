+++
governs = { root = "specs/process", glob = "*.md" }

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "placement"

[[extraction]]
primitive = "sections"
+++

# The process kind

A process spec says how the project runs — the spec system itself, evergreen
and human-authored (`specs/process/90-spec-system.md`, "the corpus is
classed"). Placement is the class: living under `specs/process/` is the
authored act that classes a file, never a filename or shape convention (same
doc, Decision). The extraction surfaces the line budget and the
heading/section structure — markdown structure only, no body-mined references
(`specs/architecture/15-kinds.md`; law 8): backtick file mentions in spec
prose are typography, never edges.

Process members publish no entities and satisfy none — the demand/claim
pairing is the intent/architecture seam; this class is orientation, governed
for placement and structure alone.
