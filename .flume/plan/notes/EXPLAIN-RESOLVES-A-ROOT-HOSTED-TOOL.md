# Root-hosted addresses parse only at explain's key reader

The fix landed in `src/member_address.rs`, not `src/address.rs` (that module is field addressing). `read.rs` needed no edit. `nested_key` now also reads `root/<kind>/<key>`, so `explain root/tool/tsc` and `explain tsc` resolve the tool. Its only callers are `read.rs`'s `resolve`.

I deliberately left `parse_nested_address`/`host_identity` alone. Their host must be a `<kind>:<name>`, and `graph.rs` reads them for containment arcs, `edge_host`, `embedded_hosts_by_key` and `node_species`. Teaching them the bare `root` host would give root tools a `("root","root")` host node in the containment family and in mention scoping, a check-visible change this entry does not cover. So there are now two readers of the grammar: one that admits the root host and one that does not. Plan should decide whether the root member gets a real address in the grammar (and whether containment arcs from root belong in the graph), then fold both readers into one.

A leaf beneath a root tool (`root/tool/tsc/package`) still parses as a leaf address whose head is `root`. Leaf resolution against it is unverified.
