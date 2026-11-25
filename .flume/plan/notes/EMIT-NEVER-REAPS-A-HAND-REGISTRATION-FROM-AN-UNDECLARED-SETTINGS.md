# Ownerless registration refusal is count-based; hand edits that keep the count stay invisible

New `OwnerlessManifestEntries` in `src/drift.rs`, raised from `unrecorded_registration_entries`.
In a container-less manifest, an entry key (`hooks.<Event>`) refuses when its on-disk groups
outnumber the prior lock's registration rows for that key AND at least one group is not one
this build renders equal (both sides go through `Manifest::parse`).

Where the lock runs out:
- Registration rows record (kind, key, manifest, key_path), with no bytes. So the lock knows
  how many groups an emit wrote under a key, but not which ones. A hand edit to an emitted
  group, or a hand group that replaces one, keeps the count and is still silently overwritten.
  Closing this needs a per-group fingerprint in the lock (or a rollup row for the ownerless
  manifest's own emit_hash). That is a lock-shape decision, so I did not invent it here.
- Conservative edge: a group the author pasted by hand that equals a newly declared member,
  next to a removed member's leftover group, refuses even though nothing would be lost.
- A lock without registration rows (P=0) makes any unmatched group refuse.
- Before this tick, the old member reap compared only the event key. So a hand group under
  an event that still has a declared member was overwritten with no ledger line at all. The
  new check closes that too (the shared-event case in the test).

Not done: `install` passes `OwnerlessManifestEntries` through and does not hoist the groups
into hook members. That was out of scope per the entry.
