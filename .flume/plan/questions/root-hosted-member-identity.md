# What address and lock shape does a root-hosted member wear?

Class: members hosted by the root (`tool`, `requirement`). `specs/model/representation.md`, "The root member", makes the root a member, but the nested-address grammar admits no bare `root` host, and the corpus names no lock shape for a root-hosted member's own leaves. Build notes found the consequences:
- two readers of the grammar: `member_address::nested_key` admits `root/<kind>/<key>`, `parse_nested_address`/`host_identity` refuse it, so a lift through `node_from_address` turns a root-hosted address into a requirement node;
- SDK `toolAddress` (`root/tool/<name>`) vs `memberAddress` (`tool:<name>`), the engine's `payload_member_address` twin;
- the root node is the pseudo-kind `(root, root)`; nothing selects it, so its out-degree is never judged;
- a requirement member reaches the lock only as a `[[declaration.requirement]]` row, so a subkind's name and leaves never reach `explain` or `check`, and `expect: [{ kind: requirement, clauses }]` binds over a population the engine does not model, a vacuous verdict nobody is told of.

Options: (a) the root gets a real grammar address, root-hosted members are `nested_member` rows, containment arcs from root enter the graph; (b) root-hosted members keep flat `<kind>:<name>` identity and the `root/` spelling retires; (c) refuse `expect` over requirement until one lands. (a) is the only one under which the subkind's leaves reach the read verbs, but it changes check-visible containment. Blocks no queued entry.
