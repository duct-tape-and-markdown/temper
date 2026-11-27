# TEST-NARRATION-CUTS

Shipped comment-only. Every `tests/<file>.rs` pointer in comments now reads
`tests/it/<file>.rs`. The sweep was scoped to comment lines, and the fixture strings
in layout_kind.rs and tool_kind.rs were left as they are. Entry-ID tags, `// ---- TAG:`
section headers, pre-fix/pre-NNNN era narration and the named wrong pointers were cut
across all named files. That covers main.rs -> gate.rs / compose, decision 0050 -> 0071,
gate_hook_fields, TypeScript -> JSON, the four-edge-loci and hand-written-mirror
claims, and the two fixture bodies' rule ids.

Not changed: install.rs:5266. The test asserts the managed-by note
(`placement::NOTE_MARKER`) both after install and after emit, so the comment there
holds and I found no modeline claim to cut. Plan's line number may point at another
site. The cli.rs :391 site likewise showed no era narration at this tick's numbering.

Outside the named files, entry-ID narration is still in comments across other suites
(rg '[A-Z]{2,}(-[A-Z]+)+' on comment lines) and in src/drift.rs's neighbours. This is
the same family, and those sites wait for a later sweep.
