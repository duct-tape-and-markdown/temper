# Path-form mention: two seams worth a plan look

Shipped `mentionOf(member, { as: "path" })` (`Mentionable.form: "path"`), resolved in
`emit.ts`'s `mentionPath` via `relativeProjection`; a deferred, embedded or registration
target refuses by name inside `emit()`, before any write.

1. **Self-citing host is circular to construct.** A skill mentioning its own supporting
   doc needs the doc value first, and the doc needs `host: skill`. The test builds a
   prose-less twin skill as the doc's host, then the real skill. It works because a host is
   read by name and facts, not identity, but an adopter writing a router (the entry's
   motivating case) hits this on the first try. Candidate: a lazy prose thunk, or a doc
   whose host is bound at composition. Design call, not build's.

2. **An embedded leaf refuses a path-form mention** (`prose.ts` `resolveLeaf`). The leaf
   lands in its host's body, so a host-relative path would mean something. But
   `declarations.ts`'s `nestedMemberRow` resolves the same leaf with no host, so supporting
   it means threading the host placement through both faces of the leaf walk. Until a spec
   asks for it, the refusal is loud rather than a divergence between fence and row.

The public `renderText` (no host) also refuses a path-form mention instead of
falling back to the name.
