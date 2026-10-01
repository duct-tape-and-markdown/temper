## Surface

`member_path_index` (`src/drift.rs:2700`) keys every member's projection path to
`host_address(&member.kind, &member.name)` while it derives that path *through*
`member.host` — so a nested **file** child is indexed under a bare
`<kind>:<name>`, the one spelling `specs/model/representation.md` ("member")
says no nested member wears.

That value is not a label: it lands in `LayoutImportRow.target`
(`resolve_source_dependency`, `src/drift.rs:2571`) and `graph`'s
`resolved_import_edges` turns it straight into a node
(`src/graph.rs:1951`, `node_from_address`). A layout region importing a nested
file child's projection therefore puts an edge to a member that does not exist
into the graph, under an address that would collide across two hosts' same-keyed
children.

This tick fixed the same mis-spelling at the five **finding** sites the entry
named (`FRESH-FINDING-NAMES-THE-MEMBERS-ADDRESS`) and left this one: it is a
declared **row** and a graph node, not a finding, so taking it would widen the
entry past its assigned surface and past its test.

## Observed at

4c0320b3 (HEAD when observed)

## Suggested consolidation

`member_path_index` already has the host in hand; route its value through the
same `member_address_under` the findings now use (`src/drift.rs`, beside
`payload_member_address`), so the index, the import row and the graph node spell
one grammar. Needs a nested-child import test and a check that no committed lock
carries a `layout_import` row targeting one.
