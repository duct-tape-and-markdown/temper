# Root-hosted address reads through one grammar

`host_identity` now reads the bare `root` host as the `(root, root)` node, so
`parse_nested_address` and everything built on it (`nested_key`,
`embedded_source_host`, `containment_edges`, `embedded_hosts_by_key`,
`member_lookup`, `node_from_address`, `node_species`) read `root/<kind>/<key>`.
`root_hosted_kind` and `nested_key`'s root branch are gone.

Behavior moved beyond containment, all toward the spec: the gate's
`member_lookup` now resolves a bare tool key (`tsc`) to `root/tool/tsc`, as
`explain` already did; a mention naming `root/tool/<key>` now binds the tool's
member node, where before it fell to a requirement node and dangled. Full suite
green with no fixture churn. No shipped root `degree` clause counts
`contains:tool` yet. That is now judgeable, but nothing judges it.
