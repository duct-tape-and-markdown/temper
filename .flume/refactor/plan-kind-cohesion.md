## Surface

`src/kind.rs` (2165 lines, model subsystem) bundles four separable jobs with
no dependency on each other's internals:

- The kind declaration algebra itself: `CustomKind`/`Governs`/`Format`/
  `UnitShape`/`Registration`/`Template`/`Content`, their row lifts — the
  module's actual job per architecture.md's codemap.
- A self-contained layout-document reader: `Layout`/`LayoutRegion`/
  `LayoutReading`/`LayoutMember`/`LayoutError`/`next_heading`/
  `read_collection_member`/`parse_edge_entries`/`slugify` (lines 284-565).
  Needs only `crate::extract` + `PathBuf`; no `CustomKind` dependency.
- The extraction-primitive algebra: `Extraction`/`Primitive`/
  `DirectiveSyntax`/`Unit` (lines ~1291-1497).
- A glob-compilation cache utility: `compile_glob`/`GLOB_CACHE`/
  `GLOB_COMPILES`/`glob_compile_count` (lines 1223-1284). Its own doc
  comment (1223-1233) names callers outside kind.rs entirely — `import`'s
  discovery walk, `coverage_note`'s `governs` leaf test, `graph`'s
  `paths-match` liveness test — so this is crate-wide glob infra sitting
  inside the kind-algebra module, not kind data.

## Observed at

78d43d3 (posture sweep, model subsystem, this tick)

## Suggested consolidation

architecture.md's Growth rules anticipate exactly this ("Splits land flat:
a cohesion finding extracts a new module into its host's subsystem, the
codemap gains a name"). The glob cache is the strongest candidate — no
dependency on kind data, multiple non-model callers already — and its
natural neighbor is `address.rs` (foundation, leaf path vocabulary),
especially once `NORMALIZE-PATH-SUBSYSTEM-PLACEMENT` (already queued)
settles path vocabulary's home there. The layout reader and extraction
algebra are more debatable (still model-shaped, harness-declaration
mechanics) — which of the four pieces actually earns its own module, and
which subsystem each lands in, is a human/plan judgment call, not
mechanical enough to derive directly into a pending entry.
