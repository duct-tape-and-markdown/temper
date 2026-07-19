<!--
Inbox — external notes for the next `plan` tick to route into pending,
open-questions, or accepted debt. Humans append lines here; plan drains and
removes them each tick. Empty is the normal state.

Stamp each note `observed at <short-sha>` — HEAD when the observation was
made — so plan can diff forward (`git log <sha>..HEAD`) instead of
re-deriving the whole premise; the queue keeps moving between filing and
routing.
-->


## Residual (field note, 9a7d7d1): round-trip gate covers one row, not the class

55b8539's round-trip test decodes exactly one derived row
(`find(|k| k.name == "installed-plugin")`) through
`CustomKind::from_kind_fact_row`; the other 13 builtin kind rows are
parsed structurally but never decoded, so a one-sided respell of any
other wire vocabulary (an `event(f)` label, a `paths-match` form, a
shape or collection-address spelling) would still pass CI the way bare
enablement did. Fix shape: replace the single `find` with a loop —
decode every row in the derived lock through `from_kind_fact_row`,
expect all 14 to succeed; keep the targeted field-carrying-Enablement
assertion as the regression pin. ~Three lines moved, no new machinery,
same test file; the gate already pays the expensive part (node build +
emit). Converts "this bug can't recur" to "this class can't recur" —
the field note's original ask. Priority low: one small entry.
