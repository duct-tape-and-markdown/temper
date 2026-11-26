# Per-member enforcement mode: what the guard resolves and what it does not

Shipped: `[[declaration.assembly]]` `mode` rows now come in three grains. A row with
neither `from` nor `member` is the root. A row with `from = <kind>` covers that kind. A
row with the new `member = <address>` column covers one member. `compose::EnforcementModes`
resolves member, then kind, then root. The SDK takes `mode` on a member init (a new
framework key, so a custom kind can no longer type a field named `mode`), on custom kind
facts, and on an `extend`/`relocate` delta. A subkind inherits its base's mode.

Not resolved per member yet (they still read the root mode). Plan should decide whether
these owe an entry:
- the PostToolUse shell edge (`main.rs`, `install::shell_edge_findings`) reports
  tree-wide findings under one mode. It would need a mode per finding, or the strictest
  mode among them.
- the co-owned manifest write binding (`install::manifest_write_findings`) acts at the
  root mode.
- `gate.rs` reads only the root mode for the dial-softening decision, so a member that
  declares `block` does not turn off its own dial softening.

`src/builtin_lock.toml` is unchanged: the embedded lock is memberless and declares no
override, so no rows moved. The frozen-lock test is green.
