## Surface

`compose::overlay_builtin_kind` (src/compose.rs:398-421) is recomputed
independently, per builtin kind, at six sites within one `gate()`
invocation — though its only inputs beyond the kind itself
(`declarations.kinds`) are provably identical at every site:
`assemble_lock_family`'s local-member fold only extends
`.nested_members`/`.satisfies`, never `.kinds` (src/compose.rs:915-930), so
`committed.kinds` and the later `declarations.kinds` never diverge.

1. `compose::assemble_lock_family` (src/compose.rs:917, via `declared_kinds`)
   — called at src/gate.rs:63, over `committed`.
2. `compose::build_manifest_cache` (src/compose.rs:1164, via
   `declared_kinds`) — called at src/gate.rs:104, over `declarations`.
3. The builtin dispatch loop's `compose::kind_units_and_features`
   (src/compose.rs:679) — called at src/gate.rs:137, once per builtin kind.
4. `compose::resolve_kind_units`'s own internal re-overlay
   (src/compose.rs:598) — invoked from inside #3's `kind_units_and_features`
   (src/compose.rs:680) on a kind #3 already overlaid, and from
   `assemble_lock_family`'s local-kind loop (src/compose.rs:921) on a kind
   #1's `declared_kinds` already overlaid. `rg` over `src/` and `tests/`
   confirms these are `resolve_kind_units`'s only two callers, both already
   passing a pre-overlaid kind — so this call is provably dead weight, not a
   differently-scoped computation: `CustomKind::overlay_templates`/
   `overlay_content` (src/kind.rs:606-626) replace wholesale rather than
   accumulate, and `row_relocates_builtin` (src/compose.rs:964-974) reads
   only `format`/`unit_shape`/`registration`, none of which overlay ever
   touches — re-deriving reproduces the identical `CustomKind` every time.
5. `admissibility::governs_collision_diagnostics` (src/admissibility.rs:
   268-269) — called at src/gate.rs:168-172.
6. `admissibility::local_locus_admissibility` (src/admissibility.rs:214-217)
   — called at src/gate.rs:176-180, immediately after #5, with the identical
   `&builtin_defs`/`&declarations` arguments.

`specs/process/engineering.md`, "Cost scale is hoisted, and pinned by
count" names exactly this class ("whole-input work computes once per run
and is shared, never recomputed per kind... or call site"), and the
codebase already carries the precedent fix for the same bug one layer up:
`RESOLVE_KIND_UNITS_COUNT`/`resolve_kind_units_count()` (src/compose.rs:
38-46) exists solely to pin that `resolve_kind_units` itself is called once
per kind, not twice — the gated test at src/main.rs:743-760 states the
prior bug in the identical shape ("resolve_kind_units was called twice per
kind: once through kind_features and again through
collect_directive_members"). That pin counts calls to `resolve_kind_units`,
never to `overlay_builtin_kind` inside it, so it does not catch this.

## Observed at

29c5baf (HEAD when observed).

## Suggested consolidation

Compute the overlaid builtin-kind map (`BTreeMap<String, CustomKind>`)
once per `gate()`/`explain()` invocation — mirroring the `ManifestCache`
precedent (`GATE-MANIFEST-SHARED-READ-HOIST`) — and thread it through
sites #1-6 instead of each re-deriving it. The open design call, not a
mechanical one: `assemble_lock_family` (src/compose.rs:909) is called at
src/gate.rs:63, before `builtin_defs`/the manifest cache/either
admissibility call exist, so the single hoist point sits earlier than any
of its current consumers, and its own signature (and `LockFamily`'s shape)
likely needs to change to carry the map out; `assemble_lock_family` has a
second call site (src/read.rs:1554, the `explain`/read verb) that would
need the same threading. #4 (`resolve_kind_units`'s dead internal overlay)
is a narrower, lower-risk sub-fix that could ship alone regardless of how
the wider hoist is decided — worth scoping as its own entry if the wider
one splits.
