## Surface

Two path normalizers for one job: src/import.rs:283 `normalize` (drops
`Component::CurDir` only) vs src/graph.rs:812 `normalize_path` (drops
`CurDir` and collapses `ParentDir` — a strict superset).

## Observed at

0ccba8d

## Suggested consolidation

One home; keep the superset. (src/coverage_note.rs:269 `normalize_root` and
src/drift.rs:620 `normalize_lf` are different jobs — name collision only,
leave them.)
