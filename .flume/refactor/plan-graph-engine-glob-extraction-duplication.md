## Surface

`graph.rs`'s `declared_globs` (src/graph.rs:780-797, doc 774-779: "The
registration globs a member declares on `field`") and `engine.rs`'s
`field_globs` (src/engine.rs:1137-1143, doc 1134-1136: "The glob strings a
field value carries — each element of a list, or a lone scalar read as a
single glob") independently reimplement the identical job — extracting the
glob strings a `FeatureValue` field carries — with a real behavioral drift:
`declared_globs` trims each glob and drops blank/whitespace-only entries
(its own doc: "Declaring none is *not* a dead edge: an absent/blank `paths`
field falls back to unconditional loading"); `field_globs` does neither
(`FeatureValue::Scalar { text, .. } => vec![text.as_str()]`,
`List(items) => items.iter().map(String::as_str).collect()`, no trim, no
empty filter). `declared_globs` is consumed by `graph.rs`'s reachability
judge (394, 398, 747); `field_globs` is consumed by `engine.rs`'s
`glob-valid` predicate (922) feeding `crate::kind::compile_glob`.

## Observed at

4baa5c4 — plan diffs forward from here.

## Suggested consolidation

One shared extractor (home: graph.rs, or a leaf helper both modules call)
that trims and filters blanks the way `declared_globs` already does, with
both call sites switched to it — but the trim/filter question needs a
design call first: does `glob-valid` currently want to flag a
blank/whitespace-only glob as a syntax problem (today it doesn't — an empty
pattern still compiles under globset), or should it silently skip blanks
the way the reachability judge already treats them as "no glob"? That call
decides whether consolidating changes glob-valid's observable findings,
which is why this rides plan rather than landing inline.
