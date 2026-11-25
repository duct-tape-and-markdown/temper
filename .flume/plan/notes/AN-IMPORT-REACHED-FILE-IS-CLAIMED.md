# Import-reached files are claimed by the coverage scan

Shipped. `graph::DirectiveClassing` gains `reached_files`: the backed, non-member
targets the import traversal reached (root-joined, normalized). The gate threads it
into `coverage_note::check`, which skips a `.claude/` file entry whose path is in it.

Observed, not taken:
- Claim is per *file* entry. A `.claude/` directory holding only import-reached files
  (e.g. `@.claude/notes/a.md`) still fires unclaimed-entry for `.claude/notes`, since
  the scan names direct children only. Whether a directory is claimed when some/all of
  its files are reached is underdetermined by `contract.md` "edge"; plan should decide.
- `read.rs` calls `classify_directives` against `Path::new(".")` and takes `.edges`
  only; it ignores the new field, which is fine today.
