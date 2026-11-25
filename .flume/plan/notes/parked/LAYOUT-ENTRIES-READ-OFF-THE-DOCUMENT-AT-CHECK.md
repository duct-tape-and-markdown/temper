# LAYOUT-ENTRIES-READ-OFF-THE-DOCUMENT-AT-CHECK: the guard half needs a ruling

The check half is entailed and small. compose::assemble_lock_family already
derives a layout document's rows at read time (resolve_kind_units ->
layout_unit -> drift::lower_layout_reading), but only for local kinds.
Committed kinds' rows are "simply unconsumed" (compose.rs layout_unit doc).
The fix is to run the same pass for committed layout kinds and replace the
lock's nested_member rows for those hosts. Both acceptance clauses then
fire, because kind_units_and_features reads the assembled family. Watch the
parse-count pins in tests/it/check_cost.rs (local-only fixture, 2 parses).

The guard half is not entailed. Today neither guard edge judges a required
clause on any layout document. Fields and entries are both unjudged, before
this change and after it. distribution.md "Per tool call" says what the
guard binds: projection drift and undeclared documents where
locus-declared is bound. Decision 0060 rejected "the whole contract instead
of the write boundary" per call. The one precedent,
install::manifest_write_findings, judges a pending co-owned manifest's
members against their contract, and only at the pre edge. So the test line
"both guard edges refuse a layout entry edit check refuses" needs a new
guard binding: contract clauses over layout documents, on the post edge
too. No spec section or Decision states one. The GH #62 report listed
"both guard edges let the edit through" as a symptom, but under the
standing corpus that is the specified behavior.

Suggested split. (1) Re-file the check half alone: tests line 1 plus a pin
that the guard edges are unchanged. (2) Raise a question for a human:
should the guard judge layout source contracts (pre edge, like manifests;
post edge, against 0060)? If the answer is yes, it is a distribution.md
amendment first.
