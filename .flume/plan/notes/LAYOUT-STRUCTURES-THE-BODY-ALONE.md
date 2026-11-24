# Layout structures the body alone — shipped

Both layout reads (check's `compose::layout_unit`, emit's `drift::read_layout_document`)
now come through one counted door, `drift::read_layout_document_member`, which reads the
document through `frontmatter::Member::from_source_rooted`: frontmatter fields reach the
unit, the id is the kind's unit shape, and the layout parses the post-frontmatter body.
A layout slot overrides a same-named frontmatter key on the unit.

Observed:
- Emit's identity: path- and file-shaped units keep the SDK name (the placement emit
  splices from it round-trips by construction); only `named-field` takes the document's
  frontmatter value. Emit passes `disk_path.parent()` as the fold base, so its computed
  file-shape id is discarded, not trusted. A real emit-side fold base (top-level:
  `fold_base(root, glob)`; nested: host unit + pattern) would let emit use the id
  unconditionally.
- `check_reads_a_relocated_builtin_document_under_its_declared_layout` relied on an
  invalid `---` fence reading as preamble; that is the very behavior this entry retires,
  so its fixture now carries valid `paths` frontmatter (a `description` key tripped the
  built-in rule's forbidden-keys clause — proof the frontmatter now reaches clauses).
- Stale prose outside this fence: `src/extract.rs` `fold_file_id` doc still says the
  layout face calls it directly; it no longer does.
- Unrefused combination: a layout kind declaring `json-document`/`toml-document` format
  dispatches to the layout (frontmatter read) silently; nothing judges layout × a
  non-frontmatter format as ill-formed.
