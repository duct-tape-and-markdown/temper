# Seam version peek reports a parse failure as PayloadParse

Shipped. `peek_and_validate_seam_version` (src/drift.rs) now takes the harness
entry and maps a non-JSON or version-less payload to `PayloadParse`; only a read
version that differs is `UnsupportedSeamVersion`. The tests[] line lives in
tests/it/emit.rs as an end-to-end `emit_program` run over a node program whose
stdout carries a stray `console.log` line ahead of the seam; verified red on the
pre-fix drift.rs and green after. The version-less case is covered by the
acceptance via the same code path but has no dedicated test.
