# A nested skill's frontmatter `name`: its leaf, or its identity?

Driver: `specs/builtins.md`, "The shipped kinds", skill. A nested skill's identity is `apps:web:deploy`, and identity comes from the directory name. The SDK skill kind's `identityField: "name"` writes the identity into frontmatter, so emit writes `name: "apps:web:deploy"`. The skill's own `allowed_chars.name` clause refuses that, and install→emit of a nested skill rewrites the `name` the author had. Seen by `SKILL-CLAIMS-SUBDIRECTORY-SKILLS`.

Two answers fit the corpus:
- frontmatter `name` is the leaf (`deploy`), and the identity is no longer the field;
- the identity stays tied to the field, and the clause or the field's role moves.

Before ruling the first, check whether Claude Code's skill docs require `name` to match the directory name. That is an external fact the corpus doesn't cite yet.
