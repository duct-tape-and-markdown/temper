# A lockless harness resolves built-in edges from the embedded lock, under 0106

This retires LOCKLESS-CHECK-RESOLVES-THE-BUILT-IN-EDGES. It was derived before 0106, and its premise ("a dangling outputStyle or marketplace is a finding") is now false: an off-tree target is never dangling (0106). Three builds were refused on test lines patched around that premise. Re-derive from the rule as it stands.

On a harness with no lock, `check` resolves the built-in assembly edges (`settings`/`settings-local` `outputStyle` → output-style; `installed-plugin` → known-marketplace) from the embedded lock:
- a target the tree holds resolves;
- a provided name (a documented built-in style) resolves;
- any other name collapses to one coverage note naming the scopes temper does not read, never a dangling finding.

The likely sites are `src/gate.rs` (lockless edge assembly) and `src/coverage_note.rs` (the note). Acceptance:
- **new:** a lockless `settings.local.json` naming a style no folder holds draws one coverage note and no `graph.route`;
- **kept:** a lockless `outputStyle: "default"` resolves;
- **kept:** a style in `.claude/output-styles/` resolves.
