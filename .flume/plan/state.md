# Plan state

- Spec derived through: a0fccaf
- Audited through: 2f9e277
- Residue swept through: 503de24
- This tick: Residue sweep. fcaf08e (the one unswept code commit) swept:
  structurally clean — layout_edge_fields (drift.rs:690) is one shared seam
  consumed by both document-read paths (emit drift.rs:567, gate main.rs:887),
  the one-job-one-home shape; edges_from_declarations (main.rs:1187) is a
  different job (graph edges, not slot names), no duplicate. One finding:
  layout_edge_fields' filter_map (drift.rs:693) silently skips a present
  edge fact missing its field column — the degrade-to-absent class
  LOCK-ROW-REJECT-LOUD files; that entry's drift.rs scope amended to name
  the site, no new entry. Six riding debts re-verified on disk, all
  unchanged (fcaf08e touched none of their files); stamps advanced.
- Queue: EMBEDDED-EDGE-TARGETS (open); BUILTIN-CONTRACT-ARRAY-SURGERY
  (blockedBy EMBEDDED-EDGE-TARGETS); LOCK-ROW-REJECT-LOUD (blockedBy
  BUILTIN-CONTRACT-ARRAY-SURGERY); PACKAGING-CHANNELS (parked).

Plan continues: yes — every input is now current; the quiet closing pass
(queue disjointness, gate reasons, state re-derivation) is the next job.
