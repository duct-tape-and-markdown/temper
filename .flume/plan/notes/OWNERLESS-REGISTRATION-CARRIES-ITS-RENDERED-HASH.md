# Ownerless registration carries its rendered hash

Shipped. Where the hash lives: an optional `emit_hash` column on the
`[[declaration.registration]]` row, written only for entries rendered into a
manifest no container member owns. It is lock-only (`serde(skip)`/`ts(skip)`),
so the seam and the SDK bindings are unchanged. I did not use a `[[<kind>]]`
provenance rollup row at the manifest's path: `prior_emit_hashes` and the gate's
whole-file drift both key provenance by path, so several entry rows at one
`settings.json` would collide with the file fingerprint. pipeline.md files this
hash under the provenance family, but here it is a column on a declaration row.
If the corpus wants a separate provenance family at entry grain, that is a lock
layout question for plan.

Hash: SHA-256 over the compact JSON of the one rendered entry (a hook matcher
group, a server object, a plugin scalar). Whitespace is never drift.

Detection (`drifted_registration_entries`) only runs at emit. `check`/`guard`
do not read the entry hash yet, so a hand edit is caught when emit runs, not at
session open. Whether the gate should report it as drift on the hook member is
for plan to decide.

Naming: the refusal names the drifted on-disk entry by its own lifted fields
(`hook:PreToolUse:Bash`). If the edit changed the matcher, the name is the edited
matcher's.
