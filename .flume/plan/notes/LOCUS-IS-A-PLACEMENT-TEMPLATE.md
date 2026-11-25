# LOCUS-IS-A-PLACEMENT-TEMPLATE

Shipped: `glob::is_placement_template` beside `compile_glob`; `CustomKind::from_kind_fact_row`
refuses a non-template `governs_glob` before lifting anything else, so every load path
(custom kinds, built-in relocation via `row_relocates_builtin`) inherits it.

Debt: `LockRowError` lives in `src/drift.rs`, outside this tick's fence, so the refusal
rides the existing `Vocabulary` variant with the rule spelled in its `column` text
(`kind::PLACEMENT_TEMPLATE_COLUMN`). The message reads "column `governs_glob (a placement
template: ...)` value `<glob>` is outside the closed vocabulary" and names neither the kind
nor a dedicated rule id. A `LockRowError::NotATemplate { kind, glob }` variant would say it
straight; worth an entry that fences `src/drift.rs`.

Validator scope: refuses `? [ ] { } \`, extglob openers (`!( @( +( *( ?(`), empty
segments, `**` past the leading segment, and more than one `*`. A literal `(`/`!` not
opening an extglob still passes. The SDK side only documents the rule (`sdk/src/kind.ts`);
it does not refuse at authoring time.
