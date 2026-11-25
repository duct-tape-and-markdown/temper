# Requirement row inherits the corpus default

Shipped as written. One default fn (`required_by_default`, src/drift.rs) now backs both readers: serde over the seam and `RequirementRow::from_table`. The table writer always emits `required`, so the absent-column test strips it from a written lock to reach the reader. The generated `RequirementRow.ts` doc changed with the field doc; the TS type stays `required: boolean` (non-optional), so the SDK's `?? true` in declarations.ts remains the only place that lowers an absent value on the write side.
