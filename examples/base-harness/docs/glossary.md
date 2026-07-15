Domain terms with a specific meaning in this repository. Each term is an
addressable member of the glossary host: a document elsewhere in the corpus
can declare a mention against it, and renaming a term is a model change the
gate sees, not a find-and-replace.

## Terms

### Corpus

The set of governed documents under `docs/` — this repository's declared
intent, and the authoritative side of every docs-versus-code disagreement.

### Source

A governed path the tool reads and never writes: a layout document like
this one, or a `.temper/` module. Every governed path is a source or a
projection, never both.

### Projection

A governed path the tool writes and never reads back for meaning:
`CLAUDE.md`, the files under `.claude/`. A hand-edit to a projection is
drift, routed to the owning source.

### Layout

A kind's declared template over a document's heading tree: prose regions,
field sections, member collections. The layout is why a section here is
model structure rather than formatting.

### Reconciliation

The standing process that closes the corpus-versus-implementation delta in
the corpus's favor. The replacement for per-change documentation duties.

### Checklist item

A line carrying a box (`- [ ]` or `- [x]`) — the unit the scanner yields
and the renderer counts. A line without a box is not an item and is never
guessed at.

