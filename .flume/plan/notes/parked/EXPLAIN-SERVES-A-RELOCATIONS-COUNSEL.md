# EXPLAIN-SERVES-A-RELOCATIONS-COUNSEL: already holds — premise contradicted

The entry assumes explain/schema serve the built-in's counsel after a
relocation because `overlay_builtin_kind` never reads the row's
guidance/cite. That overlay is the wrong seam. `CustomKind` carries no
counsel at all. Counsel reaches the read verbs off the kind-fact row
directly:

- `compose::builtin_contract` (src/compose.rs, the embedded-contract arm)
  sets `contract.guidance` from the committed `declarations.kinds` row
  of that name. explain's bare-kind and member narrations and `schema`'s
  top-level `description` (src/main.rs `kind_contract`) all read it.
- `read::narrate_kind` takes `cite` from `drift::kind_facts_over_floor`,
  which puts the committed row ahead of the embedded floor's row.

Probe on this tree: a lock carrying the embedded `skill` row with
`exclude` set (so it relocates), plus `guidance = "RELOC COUNSEL"` and
`cite = "RELOC CITE"`. `temper explain skill` printed both, and
`temper schema --kind skill` had `"description": "RELOC COUNSEL"`. So
the entry's test would pass on the base tree, and the judge reverts it.

For plan: with a relocation row that declares no counsel, explain and
schema show none. The row's `None` overwrites the built-in's counsel
(the same `builtin_contract` line). That matches the SDK's intent: the
relocation drops the base counsel, and emit projects no rule. But if
"a relocation declaring none is unchanged" was meant as "still serves
the built-in's own", that is a different behavior, and it needs a
ruling. It conflicts with RELOCATION-DECLARES-GUIDANCE's
replace-not-inherit design.

The SDK's `fieldGuidance` on a relocation has no row column (only
`guidance`/`cite` exist on `KindFactRow`). If it reaches the rule body
only, explain/schema never serve per-field relocation counsel. That may
be the real gap. It was not checked here.
