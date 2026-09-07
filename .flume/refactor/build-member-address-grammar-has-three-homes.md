## Surface

One address grammar (`representation.md`, "member": `<kind>:<name>`,
`<host-address>/<kind>/<key>`, `/<leaf>` beneath it) is written and read in three
unrelated modules, none of which knows the others:

- `src/extract.rs:host_address` — writes `<kind>:<name>`, the host half.
- `src/graph.rs:nested_address` / `graph.rs:parse_nested_address` — writes and reads
  `<host-address>/<kind>/<key>` (this tick's unification of two hand-rolled parses).
- `src/read.rs:845 parse_leaf_address` — reads the four-segment leaf spelling
  `<member>/<kind>/<key>/<child-path>`, and reads its first segment as a *bare* member
  id where the nested parser reads a `<kind>:<name>` address. The two disagree today
  about what the first segment of the same string is.

`src/address.rs` is field addressing (a clause's `field` path), not member addresses,
so it is not the existing home.

## Observed at

86aa0e94 (HEAD when observed).

## Suggested consolidation

One member-address module owning the whole grammar — writer and parser per spelling,
with the leaf tail a tail of the nested address rather than a second, differently-shaped
parse. `graph` and `read` become callers; `extract::host_address` moves in.
