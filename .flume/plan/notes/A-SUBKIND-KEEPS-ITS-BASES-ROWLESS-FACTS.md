# A subkind keeps its base's row-less facts

`resolve_subkinds` now takes the base's extraction (base primitives in order, then the
subkind's own additions) and its relationships (`from` rewritten to the subkind, a field
the subkind declares itself wins). The fix lives entirely in `compose.rs`: `lift` already
attaches the name-keyed facts to the base in `kinds`, so no change to `builtin_kind.rs`
beyond a doc line was needed.

Deliberately NOT inherited: `bare_root_file`. It is a locus fact like `governs`; a skill
subkind taking `SKILL.md` would claim the harness-root document outside its own declared
locus, against "governs a locus within its base's". If plan reads the spec as requiring it,
that is a question, not a silent addition.
