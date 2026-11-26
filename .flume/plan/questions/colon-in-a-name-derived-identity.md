# What does a name holding `:` do to a name-derived identity?

0096 reserves `:` in a path segment's identity, and emit reads `:` in an
identity as a placement separator (`src/drift.rs` `placement_in_unit`;
`sdk/src/kind.ts` `identityFieldValue`). A root output style whose
frontmatter `name` holds a `:` (`name: "Mode: Teach"`) reads back fine but
emits as nested. The memory and skill kinds avoid it by deriving identity
from the path; output-style (and any named-field kind) derives it from a
field the docs leave free.

Three answers, each consistent with 0096 and `specs/builtins.md`
(output-style): refuse the name at read (a well-formedness error naming
the `:`), escape it in the identity, or record it as a known limitation.
Nothing in intent ranks them. A second noting of the colon-bearing-name
family (first: the edgeLookupKey qualifier misread).
