+++
governs = { root = "specs", glob = "*.md" }

[[extraction]]
primitive = "line_count"

[[extraction]]
primitive = "headings"

[[extraction]]
primitive = "placement"

[[extraction]]
primitive = "references"
feature = "references"
+++

# The spec kind

A spec is temper's own governing document — one file per concern under
`specs/`, evergreen, human-authored (`specs/90-spec-system.md`). The extraction
surfaces the line budget, the heading structure, and the corpus's declared
reference syntax (backtick filenames, `` `NN-name.md` ``).

Relationships (references → spec edges) are deliberately not declared yet: the
declared `strip_suffix` normalization (`specs/15-kinds.md`, "reference
resolution is declared by the kind") ships first, so the graph never dangles on
clean input.
