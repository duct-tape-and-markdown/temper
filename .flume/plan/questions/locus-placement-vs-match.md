# Is a kind's locus glob a matcher, a placement template, or both?

Driver: GH #64. Discovery matches `locus.glob` with globset (classes, braces); placement substitutes the member name for the single `*` as text (`src/drift.rs` `splice_name`), so `domains/[a-z0-9-]*.md` places at `domains/[a-z0-9-]example.md`. An extglob such as `!(README).md` fails to compile and silently matches nothing (every caller of `compile_glob` treats a failed compile as no match). A kind cannot exclude a sibling file.

Session stance (2026-11-24): one spelling (0078). The locus states placement (`domains/*.md`, one name-bearing `*`); matching derives from it; narrowing names is the existing `allowed_chars` clause, never glob syntax; a glob that does not compile is a load error (invariant 6). Open: how a kind excludes a sibling (a README) without two kinds sharing a governs glob.
